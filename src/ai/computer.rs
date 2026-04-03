use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

/// Exchanges sent to the API per call. Full history is stored on disk.
const MAX_CONTEXT_EXCHANGES: usize = 6;

// ── API request / response types ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role:    String,
    pub content: String,
}

#[derive(Serialize)]
struct ApiRequest {
    model:      &'static str,
    max_tokens: u32,
    system:     String,
    messages:   Vec<Message>,
    stream:     bool,
}

// ── ShipComputer ──────────────────────────────────────────────────────────────

pub struct ShipComputer {
    history:  Vec<Message>,
    full_log: Vec<Message>,
    log_path: Option<PathBuf>,
    client:   reqwest::blocking::Client,
}

impl ShipComputer {
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to build HTTP client");
        ShipComputer { history: vec![], full_log: vec![], log_path: None, client }
    }

    pub fn with_log(path: PathBuf) -> Self {
        let mut computer = Self::new();
        computer.log_path = Some(path.clone());

        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(log) = serde_json::from_str::<Vec<Message>>(&data) {
                computer.full_log = log;
                let start = computer.full_log.len()
                    .saturating_sub(MAX_CONTEXT_EXCHANGES * 2);
                computer.history = computer.full_log[start..].to_vec();
            }
        }
        computer
    }

    fn save_log(&self) {
        let Some(path) = &self.log_path else { return };
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.full_log) {
            let _ = std::fs::write(path, json);
        }
    }

    pub fn full_log(&self) -> &[Message] { &self.full_log }

    pub fn ask_streaming<F>(
        &mut self,
        user_message: &str,
        system_prompt: &str,
        mut on_delta: F,
    ) -> Result<String, String>
    where
        F: FnMut(&str),
    {
        use std::io::{BufRead, BufReader};

        let api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY is not set".to_string())?;

        let user_msg = Message { role: "user".to_string(), content: user_message.to_string() };
        self.history.push(user_msg.clone());
        self.full_log.push(user_msg);

        let max_msgs = MAX_CONTEXT_EXCHANGES * 2;
        if self.history.len() > max_msgs {
            self.history.drain(..self.history.len() - max_msgs);
        }

        let request_body = ApiRequest {
            model: "claude-opus-4-6",
            max_tokens: 1024,
            system: system_prompt.to_string(),
            messages: self.history.clone(),
            stream: true,
        };

        let body = serde_json::to_string(&request_body)
            .map_err(|e| format!("Serialization error: {}", e))?;

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .body(body)
            .send()
            .map_err(|e| format!("Network error: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().unwrap_or_default();
            let msg  = serde_json::from_str::<serde_json::Value>(&text)
                .ok()
                .and_then(|v| v["error"]["message"].as_str().map(|s| s.to_string()))
                .unwrap_or_else(|| format!("HTTP {}", status));
            self.history.pop();
            self.full_log.pop();
            return Err(msg);
        }

        let mut full_text = String::new();
        let reader = BufReader::new(response);

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Stream read error: {}", e))?;
            let Some(data) = line.strip_prefix("data: ") else { continue };
            if data == "[DONE]" { break; }
            let Ok(val) = serde_json::from_str::<serde_json::Value>(data) else { continue };
            if val["type"] == "content_block_delta" && val["delta"]["type"] == "text_delta" {
                if let Some(chunk) = val["delta"]["text"].as_str() {
                    full_text.push_str(chunk);
                    on_delta(chunk);
                }
            }
        }

        let asst_msg = Message { role: "assistant".to_string(), content: full_text.clone() };
        self.history.push(asst_msg.clone());
        self.full_log.push(asst_msg);
        self.save_log();

        Ok(full_text)
    }

}
