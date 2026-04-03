use std::io::{self, Write};
use rustyline;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

/// Content width used throughout the game for text wrapping and rendering.
pub const CONTENT_WIDTH: usize = 70;

// ── ANSI colour helpers ───────────────────────────────────────────────────────

pub const R:       &str = "\x1B[0m";
pub const DIM:     &str = "\x1B[2m";
pub const BOLD:    &str = "\x1B[1m";

pub const BRED:    &str = "\x1B[91m";   // bright red   — arterial, danger
pub const BYELLOW: &str = "\x1B[93m";   // bright yellow — moderate threat, warnings
pub const BGREEN:  &str = "\x1B[92m";   // bright green  — safe, harvesting
pub const BCYAN:   &str = "\x1B[96m";   // bright cyan   — UI chrome, veins
pub const BMAGENTA:&str = "\x1B[95m";   // bright magenta — extreme threat, immune activity
pub const BWHITE:  &str = "\x1B[97m";   // bright white  — titles, emphasis

// ── Basic I/O ─────────────────────────────────────────────────────────────────

pub fn prompt(msg: &str) -> String {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    match rl.readline(msg) {
        Ok(line) => line.trim().to_string(),
        Err(_)   => String::new(),
    }
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn print_header(title: &str) {
    let title: std::borrow::Cow<str> = if title.chars().count() > 56 {
        let t: String = title.chars().take(53).collect::<String>() + "...";
        t.into()
    } else {
        title.into()
    };
    println!();
    println!("  {BRED}╔════════════════════════════════════════════════════════════╗{R}");
    println!("  {BRED}║{R}  {BWHITE}{:<56}{R}  {BRED}║{R}", title);
    println!("  {BRED}╚════════════════════════════════════════════════════════════╝{R}");
    println!();
}

pub fn print_section(title: &str) {
    let dashes = "─".repeat(55usize.saturating_sub(title.len()));
    println!();
    println!("  {BCYAN}──{R} {BOLD}{title}{R} {DIM}{dashes}{R}");
}

pub fn read_key() -> String {
    enable_raw_mode().unwrap();
    let key = loop {
        match read().unwrap() {
            Event::Key(e) => {
                if e.modifiers.contains(KeyModifiers::CONTROL)
                    && e.code == KeyCode::Char('c')
                {
                    disable_raw_mode().unwrap();
                    std::process::exit(0);
                }
                let s = match e.code {
                    KeyCode::Char(c) => c.to_lowercase().to_string(),
                    KeyCode::Enter   => "\n".to_string(),
                    KeyCode::Esc     => "q".to_string(),
                    _                => continue,
                };
                break s;
            }
            _ => continue,
        }
    };
    disable_raw_mode().unwrap();
    key
}

pub fn menu_key() -> String {
    print!("\n  > ");
    io::stdout().flush().unwrap();
    let key = read_key();
    println!("{key}");
    key
}

pub fn pause() {
    print!("\n  {DIM}[press any key]{R}");
    io::stdout().flush().unwrap();
    read_key();
    println!();
}

pub fn typewrite(text: &str) {
    use std::thread::sleep;
    use std::time::Duration;
    let mut stdout = io::stdout();
    for ch in text.chars() {
        print!("{ch}");
        stdout.flush().unwrap();
        let ms = match ch {
            '.' | '!' | '?' => 80,
            ',' | ';' | ':' => 40,
            '\n'             => 60,
            _                => 18,
        };
        sleep(Duration::from_millis(ms));
    }
    println!();
}
