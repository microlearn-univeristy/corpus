mod ai;
mod body;
mod campaign;
mod notes;
mod player;
mod save;
mod ui;

use std::io::{self, Write};

use ai::companion::{attach_logs, default_companions, Companion};
use ai::computer::ShipComputer;
use body::atlas::{build_circuit, find_node, CircuitNode, ThreatLevel};
use body::host::{generate_host, generate_host_with_profile, Host};
use campaign::Campaign;
use player::inventory::Inventory;
use player::nanovehicle::NanoVehicle;
use player::state::PlayerState;
use save::{data_dir, load_api_key, store_api_key, SavedGame};
use ui::display::*;
use ui::terminal::*;

use rand::Rng;

// ── GameState ─────────────────────────────────────────────────────────────────

struct GameState {
    player:     PlayerState,
    vehicle:    NanoVehicle,
    inventory:  Inventory,
    host:       Host,
    companions: Vec<Companion>,
    helix:      ShipComputer,
    circuit:    Vec<CircuitNode>,
}

impl GameState {
    fn current_node(&self) -> &CircuitNode {
        find_node(&self.circuit, &self.player.current_node_id)
            .expect("current node not found in circuit")
    }

    fn to_saved(&self) -> SavedGame {
        SavedGame::new(
            self.player.clone(),
            self.vehicle.clone(),
            self.inventory.clone(),
            self.host.clone(),
        )
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    ensure_api_key();

    loop {
        clear();
        print_header("CORPUS — A Nanoscale Voyage");
        println!("  {DIM}Navigate the cardiovascular system of a living human body.{R}");
        println!("  {DIM}Complete the Magellan Circuit: visit all 8 organ checkpoints{R}");
        println!("  {DIM}and return to the heart.{R}");
        print_section("MAIN MENU");
        println!();
        println!("  {BWHITE}[1]{R}  New expedition");
        println!("  {BWHITE}[2]{R}  Load expedition");
        println!("  {BWHITE}[q]{R}  Quit");

        match menu_key().as_str() {
            "1" => {
                if let Some(mut state) = new_game() {
                    game_loop(&mut state);
                }
            }
            "2" => {
                if let Some(saved) = pick_save() {
                    let mut state = state_from_save(saved);
                    game_loop(&mut state);
                }
            }
            "q" | "\n" => break,
            _ => {}
        }
    }
}

// ── API key setup ─────────────────────────────────────────────────────────────

fn ensure_api_key() {
    if std::env::var("ANTHROPIC_API_KEY").is_ok() { return; }
    if let Some(key) = load_api_key() {
        unsafe { std::env::set_var("ANTHROPIC_API_KEY", key); }
        return;
    }
    clear();
    print_header("CORPUS — API Key Required");
    println!("  {DIM}An Anthropic API key is required for AI companions.{R}");
    println!("  {DIM}Get one at console.anthropic.com{R}");
    println!();
    let key = prompt("  Enter API key: ");
    if !key.is_empty() {
        let _ = store_api_key(&key);
        unsafe { std::env::set_var("ANTHROPIC_API_KEY", &key); }
    }
}

fn pilot_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

// ── New game ──────────────────────────────────────────────────────────────────

fn new_game() -> Option<GameState> {
    let campaign = pick_campaign()?;

    clear();
    print_header(&format!("NEW EXPEDITION — {}", campaign.name));
    println!();
    println!("  {DIM}{}{R}", campaign.description);
    println!();
    let pilot_name = prompt("  Pilot name: ");
    if pilot_name.is_empty() { return None; }

    let host = match campaign.host_profile.clone() {
        Some(profile) => generate_host_with_profile(profile),
        None          => generate_host(),
    };

    let player = PlayerState::new(
        pilot_name.clone(),
        campaign.id,
        campaign.name,
        campaign.objectives.iter().map(|s| s.to_string()).collect(),
        campaign.win_node.to_string(),
    );
    let vehicle    = NanoVehicle::new(&pilot_name);
    let inventory  = Inventory::new();
    let circuit    = build_circuit();
    let mut companions = default_companions();
    let pslug = pilot_slug(&pilot_name);
    attach_logs(&mut companions, &data_dir(), &pslug);
    let helix = ShipComputer::with_log(data_dir().join("logs").join(&pslug).join("helix.json"));

    let state = GameState { player, vehicle, inventory, host, companions, helix, circuit };

    // Intro sequence
    clear();
    print_header(&campaign.name.to_uppercase());
    println!();
    for line in campaign.intro {
        typewrite(&format!("  {line}"));
        println!();
    }
    typewrite(&format!(
        "  Host: {}. Profile: {}.", state.host.name, state.host.profile.label()
    ));
    println!();
    typewrite(&format!(
        "  Heart rate: {} bpm. Blood pressure: {}. Location: Left Ventricle.",
        state.host.heart_rate, state.host.blood_pressure_display()
    ));
    println!();
    pause();

    Some(state)
}

fn pick_campaign() -> Option<Campaign> {
    let campaigns = campaign::all_campaigns();
    loop {
        clear();
        print_header("SELECT CAMPAIGN");
        println!();
        for (i, c) in campaigns.iter().enumerate() {
            println!("  {BWHITE}[{}]{R}  {BWHITE}{}{R}", i + 1, c.name);
            println!("       {DIM}{}{R}", c.tagline);
            println!();
        }
        println!("  {BWHITE}[q]{R}  Back");

        let key = menu_key();
        if key == "q" { return None; }
        let idx = match key.parse::<usize>() {
            Ok(n) if n >= 1 && n <= campaigns.len() => n - 1,
            _ => continue,
        };
        return Some(campaigns[idx].clone());
    }
}

// ── Load game ─────────────────────────────────────────────────────────────────

fn pick_save() -> Option<SavedGame> {
    let saves = save::list_saves();
    if saves.is_empty() {
        clear();
        print_header("LOAD EXPEDITION");
        println!("  {DIM}No saved expeditions found.{R}");
        pause();
        return None;
    }

    clear();
    print_header("LOAD EXPEDITION");
    println!();
    for (i, s) in saves.iter().enumerate() {
        println!(
            "  {BWHITE}[{}]{R}  {}  {DIM}— {}  {}  objectives: {}/{}{R}",
            i + 1,
            s.player.pilot_name,
            s.timestamp_display(),
            s.host.name,
            s.player.objectives_reached.len(),
            s.player.active_objectives.len(),
        );
    }
    println!();
    println!("  {BWHITE}[q]{R}  Cancel");

    let key = menu_key();
    if key == "q" { return None; }
    let idx: usize = key.parse::<usize>().ok()?.saturating_sub(1);
    saves.into_iter().nth(idx)
}

fn state_from_save(saved: SavedGame) -> GameState {
    let circuit    = build_circuit();
    let mut companions = default_companions();
    let pslug = pilot_slug(&saved.player.pilot_name);
    attach_logs(&mut companions, &data_dir(), &pslug);
    let helix = ShipComputer::with_log(data_dir().join("logs").join(&pslug).join("helix.json"));

    GameState {
        player:     saved.player,
        vehicle:    saved.vehicle,
        inventory:  saved.inventory,
        host:       saved.host,
        companions,
        helix,
        circuit,
    }
}

// ── Main game loop ────────────────────────────────────────────────────────────

fn game_loop(state: &mut GameState) {
    loop {
        if state.vehicle.is_destroyed() {
            game_over(state);
            return;
        }

        if check_mission_complete(state) {
            mission_complete(state);
            return;
        }

        let node = state.current_node().clone();
        clear();
        print_status_bar(&node, &state.vehicle, &state.inventory, &state.host);
        print_node_arrival(&node, !state.player.scanned_this_visit && !state.player.has_visited(&node.id));

        println!();
        print_section("ACTIONS");
        println!();
        println!("  {BWHITE}[s]{R}  Scan location");

        let has_resources = node.glucose_yield > 0 || node.oxygen_yield > 0;
        if has_resources && !state.player.harvested_this_visit {
            println!("  {BWHITE}[h]{R}  Harvest resources");
        }
        if state.inventory.glucose > 0 && state.inventory.oxygen > 0 {
            println!("  {BWHITE}[a]{R}  Synthesize ATP  \
                     {DIM}({} glucose + {} oxygen available){R}",
                state.inventory.glucose, state.inventory.oxygen);
        }
        println!("  {BWHITE}[n]{R}  Navigate");
        println!("  {BWHITE}[c]{R}  Comms");
        println!("  {BWHITE}[o]{R}  Objective");
        println!("  {BWHITE}[t]{R}  Atlas  {DIM}({} logged){R}", state.player.visited_nodes.len());
        println!("  {BWHITE}[v]{R}  Vessel status");
        println!("  {BWHITE}[p]{R}  Post note");
        println!("  {BWHITE}[q]{R}  Save & quit");

        match menu_key().as_str() {
            "s" => action_scan(state),
            "h" if has_resources => action_harvest(state),
            "a" if state.inventory.glucose > 0 && state.inventory.oxygen > 0 => {
                action_synthesize(state)
            }
            "n" => {
                if action_navigate(state) == NavigateResult::Quit {
                    return;
                }
            }
            "p" => action_post_note(state),
            "c" => action_comms(state),
            "o" => action_objective(state),
            "t" => action_atlas(state),
            "v" => action_vessel_status(state),
            "q" => {
                do_save(state);
                return;
            }
            _ => {}
        }
    }
}

#[derive(PartialEq)]
enum NavigateResult { Navigated, Cancelled, Quit }

// ── Actions ───────────────────────────────────────────────────────────────────

fn action_scan(state: &mut GameState) {
    let node = state.current_node().clone();
    clear();
    print_header(&format!("SCAN — {}", node.name));
    print_biology_fact(&node);

    // Record first visit
    if !state.player.has_visited(&node.id) {
        state.player.visit_node(&node.id);
        println!();
        println!("  {BGREEN}✦ Logged to Atlas.{R}");
    }
    state.player.scanned_this_visit = true;
    state.player.record_objective(&node.id);

    if state.player.objectives_reached.contains(&node.id.to_string())
        && state.player.active_objectives.contains(&node.id.to_string())
    {
        println!();
        println!(
            "  {BYELLOW}✦ OBJECTIVE: {} confirmed.{R}",
            node.name
        );
        println!(
            "  {DIM}Mission progress: {}/{}{R}",
            state.player.objectives_reached.len(),
            state.player.active_objectives.len()
        );
    }

    pause();
}

fn action_harvest(state: &mut GameState) {
    let node = state.current_node().clone();
    clear();
    print_header(&format!("HARVEST — {}", node.name));

    let gluc = ((node.glucose_yield as f64) * state.host.profile.glucose_multiplier()) as u32;
    let oxy  = ((node.oxygen_yield  as f64) * state.host.profile.oxygen_multiplier())  as u32;

    state.inventory.glucose += gluc;
    state.inventory.oxygen  += oxy;
    state.player.harvested_this_visit = true;

    println!();
    if gluc > 0 {
        println!("  {BGREEN}+ {} glucose harvested.{R}", gluc);
    }
    if oxy > 0 {
        println!("  {BGREEN}+ {} oxygen harvested.{R}", oxy);
    }
    println!();
    println!("  {DIM}Inventory — glucose: {}  oxygen: {}  ATP: {}{R}",
        state.inventory.glucose, state.inventory.oxygen, state.inventory.atp);

    pause();
}

fn action_synthesize(state: &mut GameState) {
    let gained = state.inventory.synthesize_atp();
    clear();
    print_header("ATP SYNTHESIS");
    println!();
    if gained > 0 {
        println!("  {BGREEN}+ {} ATP synthesized from glucose + oxygen.{R}", gained);
        println!("  {DIM}ATP reserve: {}{R}", state.inventory.atp);
    } else {
        println!("  {BYELLOW}ATP reserve is already full, or insufficient reagents.{R}");
    }
    pause();
}

fn action_navigate(state: &mut GameState) -> NavigateResult {
    let node = state.current_node().clone();
    clear();
    print_header(&format!("NAVIGATE FROM: {}", node.name));
    print_connections(&state.circuit, node.connections, &state.player.visited_nodes);
    println!();
    println!("  {BWHITE}[q]{R}  Cancel");

    let key = menu_key();
    if key == "q" { return NavigateResult::Cancelled; }

    let idx = match key.parse::<usize>() {
        Ok(n) if n >= 1 && n <= node.connections.len() => n - 1,
        _ => return NavigateResult::Cancelled,
    };

    let target_id = node.connections[idx];
    let target = match find_node(&state.circuit, target_id) {
        Some(n) => n.clone(),
        None    => return NavigateResult::Cancelled,
    };

    // Spend ATP
    if !state.inventory.spend_atp(target.atp_cost) {
        println!();
        println!("  {BRED}Insufficient ATP to navigate there ({} required, {} available).{R}",
            target.atp_cost, state.inventory.atp);
        pause();
        return NavigateResult::Cancelled;
    }

    // Move
    state.player.current_node_id = target_id.to_string();
    state.player.visit_node(target_id);
    state.player.record_objective(target_id);

    // Apply threat damage
    apply_threat(state, &target);

    NavigateResult::Navigated
}

fn apply_threat(state: &mut GameState, node: &CircuitNode) {
    // Apply host profile modifier: shift threat level up or down, clamped to [Minimal, Extreme]
    let raw = node.threat_level as i32 + state.host.profile.threat_modifier();
    let effective_threat = match raw.clamp(0, 4) {
        0 => ThreatLevel::Minimal,
        1 => ThreatLevel::Low,
        2 => ThreatLevel::Moderate,
        3 => ThreatLevel::High,
        _ => ThreatLevel::Extreme,
    };

    let base_dmg = effective_threat.hull_damage_pct();
    if base_dmg == 0.0 { return; }

    let mut rng = rand::rng();

    // Randomize: 50–150% of base damage
    let roll: f64 = rng.random_range(50u32..=150u32) as f64 / 100.0;
    let dmg = base_dmg * roll;

    // Extreme always damages; others have a chance to avoid
    let hits = match effective_threat {
        ThreatLevel::Extreme  => true,
        ThreatLevel::High     => rng.random_bool(0.7),
        ThreatLevel::Moderate => rng.random_bool(0.35),
        _                     => false,
    };

    if hits {
        state.vehicle.apply_damage(dmg);
        clear();
        print_header("IMMUNE CONTACT");
        println!();

        let msg = match effective_threat {
            ThreatLevel::Extreme =>
                "CRITICAL — Blood-brain barrier microglia have breached the hull. \
                 Antibody clusters forming on the vessel exterior.",
            ThreatLevel::High =>
                "Kupffer cells / immune progenitors engaging the vessel. \
                 Hull sustaining phagocytic contact damage.",
            _ =>
                "Immune surveillance detected. Minor hull abrasion from \
                 macrophage contact.",
        };
        println!("  {BMAGENTA}{msg}{R}");
        println!();
        println!("  {BRED}Hull integrity: {}{R}", state.vehicle.hull_display());
        pause();
    }
}

fn action_comms(state: &mut GameState) {
    clear();
    print_header("COMMS");
    println!();
    for (i, c) in state.companions.iter().enumerate() {
        println!("  {BWHITE}[{}]{R}  {}  {DIM}— {}{R}", i + 1, c.name, c.role);
    }
    println!("  {BWHITE}[h]{R}  HELIX  {DIM}— ship AI, biological analyst{R}");
    println!("  {BWHITE}[q]{R}  Close");

    match menu_key().as_str() {
        "1" => companion_chat(state, 0),
        "2" => companion_chat(state, 1),
        "h" => helix_chat(state),
        _   => {}
    }
}

fn md_skin() -> termimad::MadSkin {
    use termimad::crossterm::style::Color as TC;
    let mut skin = termimad::MadSkin::default();
    skin.bold.set_fg(TC::Yellow);
    skin.italic.set_fg(TC::Magenta);
    for h in &mut skin.headers { h.set_fg(TC::Cyan); }
    skin.inline_code.set_fg(TC::Green);
    skin
}

fn stream_and_render<F>(result: Result<String, String>, get_text: F)
where
    F: FnOnce() -> String,
{
    match result {
        Ok(_) => {
            let skin = md_skin();
            println!();
            print!("{}", skin.text(&get_text(), Some(CONTENT_WIDTH)));
            println!();
        }
        Err(e) => println!("  {BRED}[comms error: {e}]{R}\n"),
    }
}

fn companion_chat(state: &mut GameState, idx: usize) {
    let node = state.current_node().clone();
    let skin = md_skin();
    loop {
        clear();
        let name = state.companions[idx].name;
        print_header(&format!("COMMS — {}", name));
        println!("  {DIM}[q] to close{R}");
        println!();

        // Show last few exchanges
        let log = state.companions[idx].computer.full_log().to_vec();
        let start = log.len().saturating_sub(6);
        for msg in &log[start..] {
            if msg.role == "user" {
                println!("  {BCYAN}You:{R}  {}", msg.content);
                println!();
            } else {
                println!("  {BWHITE}{}:{R}", name);
                print!("{}", skin.text(&msg.content, Some(CONTENT_WIDTH)));
                println!();
            }
        }

        let input = prompt("  You: ");
        if input.is_empty() || input == "q" { break; }

        let system = companion_system_prompt(state, idx, &node);
        print!("  {BWHITE}{}:{R}\n", name);
        io::stdout().flush().unwrap();

        let start_row = crossterm::cursor::position().ok().map(|(_, r)| r);

        let result = state.companions[idx].computer.ask_streaming(&input, &system, |chunk| {
            for ch in chunk.chars() {
                print!("{ch}");
                io::stdout().flush().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        if let Some(row) = start_row {
            use crossterm::{execute, cursor, terminal};
            execute!(io::stdout(), cursor::MoveTo(0, row), terminal::Clear(terminal::ClearType::FromCursorDown)).ok();
        } else {
            println!();
        }

        let last = state.companions[idx].computer.full_log().last()
            .map(|m| m.content.clone())
            .unwrap_or_default();
        stream_and_render(result, || last);
    }
}

fn helix_chat(state: &mut GameState) {
    let node = state.current_node().clone();
    let skin = md_skin();
    loop {
        clear();
        print_header("COMMS — HELIX");
        println!("  {DIM}[q] to close{R}");
        println!();

        let log = state.helix.full_log().to_vec();
        let start = log.len().saturating_sub(6);
        for msg in &log[start..] {
            if msg.role == "user" {
                println!("  {BCYAN}You:{R}  {}", msg.content);
                println!();
            } else {
                println!("  {BGREEN}HELIX:{R}");
                print!("{}", skin.text(&msg.content, Some(CONTENT_WIDTH)));
                println!();
            }
        }

        let input = prompt("  You: ");
        if input.is_empty() || input == "q" { break; }

        let system = helix_system_prompt(state, &node);
        print!("  {BGREEN}HELIX:{R}\n");
        io::stdout().flush().unwrap();

        let start_row = crossterm::cursor::position().ok().map(|(_, r)| r);

        let result = state.helix.ask_streaming(&input, &system, |chunk| {
            for ch in chunk.chars() {
                print!("{ch}");
                io::stdout().flush().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        if let Some(row) = start_row {
            use crossterm::{execute, cursor, terminal};
            execute!(io::stdout(), cursor::MoveTo(0, row), terminal::Clear(terminal::ClearType::FromCursorDown)).ok();
        } else {
            println!();
        }

        let last = state.helix.full_log().last()
            .map(|m| m.content.clone())
            .unwrap_or_default();
        stream_and_render(result, || last);
    }
}

fn action_objective(state: &mut GameState) {
    clear();
    print_header(&state.player.campaign_name.to_uppercase());
    println!();
    println!("  {DIM}{}{R}", state.player.campaign_name);
    println!();

    let circuit = &state.circuit;
    for obj_id in &state.player.active_objectives {
        let reached  = state.player.objectives_reached.contains(obj_id);
        let name     = circuit.iter().find(|n| n.id == *obj_id).map(|n| n.name).unwrap_or(obj_id);
        let mark     = if reached { format!("{BGREEN}✓{R}") } else { format!("{DIM}○{R}") };
        println!("  {}  {}", mark, name);
    }

    println!();
    let done  = state.player.objectives_reached.len();
    let total = state.player.active_objectives.len();
    println!("  {DIM}Progress: {BWHITE}{}/{}{R}", done, total);

    if done == total {
        println!();
        println!(
            "  {BGREEN}All objectives complete. Return to the Left Ventricle to finish.{R}"
        );
    }

    println!();
    println!("  {DIM}Current location: {BWHITE}{}{R}", state.current_node().name);
    pause();
}

fn action_atlas(state: &mut GameState) {
    clear();
    print_header("ANATOMICAL ATLAS");
    print_atlas(&state.circuit, &state.player);
    pause();
}

fn action_post_note(state: &mut GameState) {
    let node = state.current_node().clone();
    clear();
    print_header("POST NOTE");

    if !notes::is_configured() {
        println!("  {BYELLOW}Study-tools is not configured.{R}");
        println!();
        println!("  Set these environment variables to enable:");
        println!("  {DIM}STUDY_TOOLS_URL{R}  — base URL of your study-tools instance");
        println!("  {DIM}STUDY_TOOLS_KEY{R}  — your ingest API key");
        println!();
        pause();
        return;
    }

    println!("  {DIM}Location : {}{R}", node.name);
    println!("  {DIM}Campaign : {}{R}", state.player.campaign_name);
    println!();
    println!("  {DIM}Enter your note. Blank line to send, [q] alone to cancel.{R}");
    println!();

    let mut lines: Vec<String> = Vec::new();
    loop {
        let line = prompt(&format!("  {DIM}>{R} "));
        if line == "q" && lines.is_empty() {
            return;
        }
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }

    if lines.is_empty() {
        println!("\n  {DIM}No note sent.{R}");
        pause();
        return;
    }

    let body = lines.join("\n");
    let source = format!(
        "corpus:{}:{}",
        state.player.campaign_id,
        node.id,
    );
    let tags = vec!["corpus", state.player.campaign_id.as_str()];

    print!("\n  {DIM}Transmitting...{R}");
    io::stdout().flush().unwrap();

    match notes::send_note(&body, &source, tags) {
        Ok(()) => println!("  {BGREEN}Note saved to notebook.{R}"),
        Err(e) => println!("  {BRED}Failed: {e}{R}"),
    }
    pause();
}

fn action_vessel_status(state: &mut GameState) {
    clear();
    print_header("VESSEL STATUS");
    println!();

    let hull_pct = (state.vehicle.hull_integrity * 100.0).round() as u32;
    let hull_color = if hull_pct > 60 { BGREEN }
                     else if hull_pct > 30 { BYELLOW }
                     else { BRED };

    println!("  {DIM}Vessel name:   {BWHITE}{}{R}", state.vehicle.name);
    println!("  {DIM}Hull integrity:{R} {hull_color}{}%{R}", hull_pct);
    println!();
    println!("  {DIM}ATP reserve:   {BCYAN}{}{R}", state.inventory.atp);
    println!("  {DIM}Glucose:       {BGREEN}{}{R}", state.inventory.glucose);
    println!("  {DIM}Oxygen:        {BGREEN}{}{R}", state.inventory.oxygen);
    println!();
    println!("  {DIM}Host:          {BWHITE}{}{R}", state.host.name);
    println!("  {DIM}Profile:       {BYELLOW}{}{R}", state.host.profile.label());
    println!("  {DIM}Heart rate:    {R}{} bpm", state.host.heart_rate);
    println!("  {DIM}Blood pressure:{R} {}", state.host.blood_pressure_display());
    println!();
    println!("  {DIM}Atlas entries: {R}{}", state.player.visited_nodes.len());
    println!("  {DIM}Objectives:    {R}{}/{}",
        state.player.objectives_reached.len(), state.player.active_objectives.len());

    pause();
}

// ── System prompts ────────────────────────────────────────────────────────────

fn companion_system_prompt(state: &GameState, idx: usize, node: &CircuitNode) -> String {
    let c = &state.companions[idx];
    let host = &state.host;
    format!(
        "{}\n\n\
         CURRENT SITUATION:\n\
         - Location: {} ({})\n\
         - Threat level: {}\n\
         - Hull integrity: {}\n\
         - Host: {} ({})\n\
         - Campaign: {}\n\
         - Objectives reached: {}/{}\n\
         - ATP remaining: {}\n\n\
         You are speaking directly with your pilot, {}.",
        c.personality,
        node.name, node.node_type.label(),
        node.threat_level.label(),
        state.vehicle.hull_display(),
        host.name, host.profile.label(),
        state.player.campaign_name,
        state.player.objectives_reached.len(), state.player.active_objectives.len(),
        state.inventory.atp,
        state.player.pilot_name,
    )
}

fn helix_system_prompt(state: &GameState, node: &CircuitNode) -> String {
    let host = &state.host;
    format!(
        "You are HELIX — Heuristic Environmental Life-form Information eXpert — \
         the AI intelligence embedded in this nanoscale vessel. You provide environmental \
         data, biological analysis, and navigation guidance. You are clinical, precise, \
         and concise. You have a dry manner that occasionally borders on dark humor, \
         but you are never wrong about facts. When you don't know something, you say so.\n\n\
         CURRENT SENSOR DATA:\n\
         - Location: {} ({})\n\
         - Node description: {}\n\
         - Immune threat level: {}\n\
         - Hull integrity: {}\n\
         - ATP: {}  Glucose: {}  Oxygen: {}\n\
         - Host: {} — {} — HR: {} bpm, BP: {}\n\
         - Campaign: {}\n\
         - Objectives logged: {}/{}\n\n\
         Respond concisely. The pilot's name is {}.",
        node.name, node.node_type.label(),
        node.description,
        node.threat_level.label(),
        state.vehicle.hull_display(),
        state.inventory.atp, state.inventory.glucose, state.inventory.oxygen,
        host.name, host.profile.label(), host.heart_rate, host.blood_pressure_display(),
        state.player.campaign_name,
        state.player.objectives_reached.len(), state.player.active_objectives.len(),
        state.player.pilot_name,
    )
}

// ── Win / Loss conditions ─────────────────────────────────────────────────────

fn check_mission_complete(state: &mut GameState) -> bool {
    if state.player.mission_complete { return true; }
    if state.player.objectives_complete()
        && state.player.current_node_id == state.player.win_node
    {
        state.player.mission_complete = true;
        return true;
    }
    false
}

fn mission_complete(state: &mut GameState) {
    let campaigns = campaign::all_campaigns();
    let win_text = campaigns.iter()
        .find(|c| c.id == state.player.campaign_id)
        .map(|c| c.win_text)
        .unwrap_or(&["Mission complete.", "Well done.", ""]);

    clear();
    print_header("MISSION COMPLETE");
    println!();
    for line in win_text {
        if !line.is_empty() {
            typewrite(&format!("  {line}"));
            println!();
        }
    }
    println!("  {BGREEN}Objectives:    {}/{}{R}",
        state.player.active_objectives.len(), state.player.active_objectives.len());
    println!("  {BGREEN}Atlas entries: {}{R}", state.player.visited_nodes.len());
    println!("  {BGREEN}Hull remaining:{R} {}", state.vehicle.hull_display());
    println!();
    pause();
    do_save(state);
}

fn game_over(state: &mut GameState) {
    clear();
    print_header("VESSEL DESTROYED");
    println!();
    typewrite(&format!(
        "  Pilot {}. Hull integrity zero. The immune system has done its work.",
        state.player.pilot_name
    ));
    println!();
    typewrite(&format!(
        "  You were destroyed in: {}.",
        state.current_node().name
    ));
    println!();
    typewrite(
        "  The body will not remember you. The phagocytes will digest what remains \
         and the host will continue, unaware.",
    );
    println!();
    println!("  {DIM}Objectives reached: {}/{}{R}",
        state.player.objectives_reached.len(), state.player.active_objectives.len());
    pause();
}

fn do_save(state: &GameState) {
    let saved = state.to_saved();
    match save::save(&saved) {
        Ok(_)  => {},
        Err(e) => eprintln!("Save failed: {e}"),
    }
}
