use std::{fs, io, path::PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::player::state::PlayerState;
use crate::player::nanovehicle::NanoVehicle;
use crate::player::inventory::Inventory;
use crate::body::host::Host;

// ── Paths ─────────────────────────────────────────────────────────────────────

pub fn data_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".corpus")
}

fn saves_dir() -> PathBuf { data_dir().join("saves") }

pub fn api_key_path() -> PathBuf { data_dir().join("api_key") }

fn ensure_dirs() -> io::Result<()> {
    fs::create_dir_all(saves_dir())
}

// ── SavedGame ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedGame {
    pub slot:     String,
    pub saved_at: u64,
    pub player:   PlayerState,
    pub vehicle:  NanoVehicle,
    pub inventory: Inventory,
    pub host:     Host,
}

impl SavedGame {
    pub fn new(
        player:    PlayerState,
        vehicle:   NanoVehicle,
        inventory: Inventory,
        host:      Host,
    ) -> Self {
        let saved_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let slot = player.pilot_name.clone();
        SavedGame { slot, saved_at, player, vehicle, inventory, host }
    }

    pub fn timestamp_display(&self) -> String {
        let secs = self.saved_at;
        let s    = secs % 60;
        let m    = (secs / 60) % 60;
        let h    = (secs / 3600) % 24;
        let days = secs / 86400;
        let (y, mo, d) = days_to_ymd(days);
        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", y, mo, d, h, m, s)
    }
}

fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    let mut d = days + 719468;
    let era  = d / 146097;
    d %= 146097;
    let yoe = (d - d/1460 + d/36524 - d/146096) / 365;
    let y   = yoe + era * 400;
    let doy = d - (365*yoe + yoe/4 - yoe/100);
    let mp  = (5*doy + 2) / 153;
    let dom = doy - (153*mp + 2)/5 + 1;
    let mo  = if mp < 10 { mp + 3 } else { mp - 9 };
    let y   = if mo <= 2 { y + 1 } else { y };
    (y, mo, dom)
}

// ── Save / Load ───────────────────────────────────────────────────────────────

pub fn save(game: &SavedGame) -> io::Result<()> {
    ensure_dirs()?;
    let path = saves_dir().join(format!("{}.json", sanitize_slot(&game.slot)));
    let json = serde_json::to_string_pretty(game)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(path, json)
}

pub fn list_saves() -> Vec<SavedGame> {
    let dir = saves_dir();
    if !dir.exists() { return vec![]; }

    let mut saves: Vec<SavedGame> = fs::read_dir(&dir)
        .into_iter().flatten().flatten()
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
        .filter_map(|e| {
            let json = fs::read_to_string(e.path()).ok()?;
            serde_json::from_str(&json).ok()
        })
        .collect();

    saves.sort_by(|a, b| b.saved_at.cmp(&a.saved_at));
    saves
}

fn sanitize_slot(slot: &str) -> String {
    slot.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

// ── API key ───────────────────────────────────────────────────────────────────

pub fn load_api_key() -> Option<String> {
    fs::read_to_string(api_key_path())
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn store_api_key(key: &str) -> io::Result<()> {
    ensure_dirs()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let mut opts = fs::OpenOptions::new();
        opts.write(true).create(true).truncate(true).mode(0o600);
        use io::Write;
        opts.open(api_key_path())?.write_all(key.trim().as_bytes())?;
        return Ok(());
    }
    #[cfg(not(unix))]
    fs::write(api_key_path(), key.trim().as_bytes())
}
