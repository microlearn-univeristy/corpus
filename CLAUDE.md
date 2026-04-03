# Corpus — Claude Code Guide

## Build & Run

```bash
cargo build           # dev build
cargo build --release # release build
cargo run             # run
```

Requires `ANTHROPIC_API_KEY` in the environment, or the game will prompt on first run
and store it at `~/.corpus/api_key`.

## Project Structure

```
src/
  main.rs               # game loop, all menus, GameState
  save.rs               # save/load (JSON via serde → ~/.corpus/saves/)
  ai/
    companion.rs        # Dr. Mara Yun (immunologist) and Theo (cell biologist)
    computer.rs         # ShipComputer — wraps Anthropic API (streaming + non-streaming)
    mod.rs
  ui/
    terminal.rs         # ANSI helpers, prompt(), read_key(), menu_key(), typewrite()
    display.rs          # print_status_bar(), print_node_arrival(), print_biology_fact(), etc.
    mod.rs
  body/
    atlas.rs            # CircuitNode definitions, ThreatLevel, build_circuit(), CHECKPOINTS
    host.rs             # Host, HostProfile, generate_host()
    mod.rs
  player/
    state.rs            # PlayerState — current node, visited nodes, checkpoints
    nanovehicle.rs      # NanoVehicle — hull integrity, shielding
    inventory.rs        # Inventory — ATP, glucose, oxygen + synthesis logic
    mod.rs
```

## Key Concepts

### The Magellan Circuit
The win condition: visit all 8 checkpoint nodes and return to `left_ventricle`.
Checkpoints are defined in `body/atlas::CHECKPOINTS`:
`heart_muscle`, `brain`, `lungs`, `liver`, `small_intestine`, `kidneys`, `bone_marrow`, `skeletal_muscle`.

### Navigation
The body is modeled as a graph of `CircuitNode`s connected bidirectionally.
`node.connections` lists adjacent node IDs. Movement costs `node.atp_cost` ATP.
If ATP hits 0, the player cannot move.

### Fuel Loop
- Glucose harvested at: `small_intestine` (20), `liver` (10), `bone_marrow` (8), others smaller
- Oxygen harvested at: `lungs` (25), `pulmonary_vein` (5), `skeletal_muscle` (5)
- Synthesis: 1 glucose + 1 oxygen → 10 ATP (`player/inventory.rs::ATP_PER_SYNTHESIS`)

### Immune Threat
`ThreatLevel` on each node maps to hull damage on arrival:
- Moderate: 3% base, 35% chance
- High: 12% base, 70% chance
- Extreme: 30% base, guaranteed

Host profile modifies the environment: `Infected` adds 1 threat tier everywhere,
`Athlete` subtracts 1 (see `body/host.rs::threat_modifier` — not yet wired into `apply_threat`,
a good next task).

### AI Companions
Same pattern as cosmic-sim. System prompt is rebuilt every call with current game state
(location, hull, checkpoints, host profile). Dr. Mara Yun = immunologist, anxious about
threats. Theo = enthusiastic young cell biologist, sometimes wrong. HELIX = ship AI,
clinical and dry.

### Input
- `menu_key()` — single keypress (no Enter), crossterm raw mode
- `prompt()` — full line input via rustyline, used for free text / companion chat
- `typewrite()` — character-by-character narration with punctuation pauses

### Save System
`save.rs` serializes `SavedGame` to `~/.corpus/saves/{pilot_name}.json`.
Companion logs → `~/.corpus/logs/{companion}.json`. Auto-saved on quit.

## Things to Know

- `main.rs` carries all game flow; natural refactor would be splitting menus into `ui/`
- Host profile threat modifier is computed but not yet applied in `apply_threat()` — wire it in
- No atlas display screen yet — `visited_nodes` is tracked, display is a natural next feature
- `ShipComputer::ask()` (non-streaming) exists but game currently uses `ask_streaming` everywhere
