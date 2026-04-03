use crate::body::atlas::{CircuitNode, ThreatLevel};
use crate::player::inventory::Inventory;
use crate::player::nanovehicle::NanoVehicle;
use crate::player::state::PlayerState;
use crate::body::host::Host;
use crate::ui::terminal::*;

pub fn threat_color(level: ThreatLevel) -> &'static str {
    match level {
        ThreatLevel::Minimal  => BGREEN,
        ThreatLevel::Low      => BGREEN,
        ThreatLevel::Moderate => BYELLOW,
        ThreatLevel::High     => BRED,
        ThreatLevel::Extreme  => BMAGENTA,
    }
}

pub fn print_status_bar(
    node:    &CircuitNode,
    vehicle: &NanoVehicle,
    inv:     &Inventory,
    host:    &Host,
) {
    let tc = threat_color(node.threat_level);
    let hull_pct = (vehicle.hull_integrity * 100.0).round() as u32;
    let hull_color = if hull_pct > 60 { BGREEN }
                     else if hull_pct > 30 { BYELLOW }
                     else { BRED };

    println!();
    println!(
        "  {DIM}┌─ {BWHITE}{}{R}{DIM} ─── {R}{tc}{}{R}{DIM} threat{R}",
        node.name, node.threat_level.label()
    );
    println!(
        "  {DIM}│  hull {hull_color}{hull_pct}%{R}  {DIM}atp {BCYAN}{}{R}  \
         {DIM}glucose {BGREEN}{}{R}  {DIM}O₂ {BGREEN}{}{R}  \
         {DIM}host: {BYELLOW}{}{R}",
        inv.atp, inv.glucose, inv.oxygen, host.name
    );
    println!("  {DIM}└─────────────────────────────────────────{R}");
}

pub fn print_node_arrival(node: &CircuitNode, is_first_visit: bool) {
    let tc = threat_color(node.threat_level);
    println!();
    println!("  {tc}▶  {BWHITE}{}{R}  {DIM}[{}]{R}", node.name, node.node_type.label());
    println!();
    // Word-wrap description at ~70 chars
    print_wrapped("  ", node.description, CONTENT_WIDTH);
    if is_first_visit {
        println!();
        println!("  {BGREEN}✦ First visit — use [s]can to record this location.{R}");
    }
}

pub fn print_biology_fact(node: &CircuitNode) {
    println!();
    println!("  {BCYAN}── HELIX SCAN: {}{R}", node.name.to_uppercase());
    println!();
    print_wrapped("  ", node.biology, CONTENT_WIDTH);
}

pub fn print_connections(circuit: &[CircuitNode], connections: &[&str], visited: &[String]) {
    print_section("NAVIGATION");
    println!();
    for (i, id) in connections.iter().enumerate() {
        if let Some(target) = circuit.iter().find(|n| n.id == *id) {
            let tc      = threat_color(target.threat_level);
            let visited_mark = if visited.contains(&target.id.to_string()) {
                format!("{DIM}*{R}")
            } else {
                " ".to_string()
            };
            println!(
                "  {DIM}[{BWHITE}{}{DIM}]{R} {}  {BWHITE}{:<28}{R}  \
                 {DIM}type: {R}{:<14}  {DIM}threat: {tc}{}{R}  \
                 {DIM}cost: {BCYAN}{} ATP{R}",
                i + 1,
                visited_mark,
                target.name,
                target.node_type.label(),
                target.threat_level.label(),
                target.atp_cost,
            );
        }
    }
}

pub fn print_atlas(circuit: &[CircuitNode], player: &PlayerState) {
    print_section("ANATOMICAL ATLAS");
    println!();
    println!(
        "  {DIM}Locations logged: {BWHITE}{}/{}{R}",
        player.visited_nodes.len(),
        circuit.len()
    );
    println!();

    // Group by node type order: HeartChamber, Artery, Vein, Organ
    use crate::body::atlas::NodeType;
    let groups: &[(&str, NodeType)] = &[
        ("Heart Chambers", NodeType::HeartChamber),
        ("Arteries",       NodeType::Artery),
        ("Veins",          NodeType::Vein),
        ("Organs",         NodeType::Organ),
    ];

    for (label, ntype) in groups {
        let nodes: Vec<&CircuitNode> = circuit.iter()
            .filter(|n| n.node_type == *ntype)
            .collect();
        if nodes.is_empty() { continue; }

        println!("  {DIM}── {label} ─────────────────────────────────{R}");
        for node in nodes {
            let visited = player.visited_nodes.contains(&node.id.to_string());
            let is_obj  = player.active_objectives.contains(&node.id.to_string());
            let mark = if visited {
                format!("{BGREEN}✓{R}")
            } else {
                format!("{DIM}○{R}")
            };
            let cp_badge = if is_obj { format!("  {BYELLOW}[OBJECTIVE]{R}") } else { String::new() };
            println!("  {}  {:<28}{}", mark, node.name, cp_badge);
        }
        println!();
    }
}

/// Simple word-wrap printer.
fn print_wrapped(indent: &str, text: &str, width: usize) {
    let mut line_len = 0usize;
    let mut line = String::new();
    for word in text.split_whitespace() {
        if line_len + word.len() + 1 > width && !line.is_empty() {
            println!("{indent}{line}");
            line.clear();
            line_len = 0;
        }
        if !line.is_empty() {
            line.push(' ');
            line_len += 1;
        }
        line.push_str(word);
        line_len += word.len();
    }
    if !line.is_empty() {
        println!("{indent}{line}");
    }
}
