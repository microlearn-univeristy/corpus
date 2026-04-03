# Corpus

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A terminal-based game where you pilot a nanovehicle through the human circulatory system. Navigate from the heart outward through organs and vessels, managing ATP fuel and hull integrity against immune threats, until you've completed the Magellan Circuit — all 8 checkpoint organs visited and back home to the left ventricle.

## Features

- **The Magellan Circuit** — visit 8 checkpoint organs: heart muscle, brain, lungs, liver, small intestine, kidneys, bone marrow, and skeletal muscle
- **Fuel loop** — harvest glucose and oxygen from organs, synthesize ATP to power movement
- **Immune threats** — each node carries a threat level that damages your hull on arrival; host profile shifts the danger up or down
- **AI companions** — Dr. Mara Yun (immunologist), Theo (cell biologist), and HELIX (ship AI) respond to your situation via the Anthropic API
- **Host profiles** — Infected, Athlete, Diabetic, and Healthy hosts change the game environment
- **Save system** — auto-saves on quit, restores your last position and companion logs

## Requirements

- An [Anthropic API key](https://console.anthropic.com/)
- A terminal with ANSI color support

## Installation

### Pre-built binaries

Download the latest release for your platform from the [Releases](../../releases) page.

**macOS note:** the binaries are unsigned, so macOS will show a security warning on first run. To clear it:

```bash
xattr -d com.apple.quarantine corpus-macos-arm64
# or for Intel:
xattr -d com.apple.quarantine corpus-macos-x86_64
```

### Build from source

```bash
cargo build --release
./target/release/corpus
```

## Setup

On first run, the game will prompt you for your Anthropic API key if `ANTHROPIC_API_KEY` is not set in your environment. The key is stored locally at `~/.corpus/api_key` for future sessions.

## Gameplay

```
[1] Navigate         — move to an adjacent node in the circulatory system
[2] Harvest          — collect glucose or oxygen from the current location
[3] Synthesize ATP   — convert glucose + oxygen into fuel (10 ATP per unit)
[4] Ship status      — hull integrity, shielding, inventory
[c] Companions       — talk to Dr. Mara Yun or Theo
[h] Hail HELIX       — ask your ship AI about your current situation
```

**Objective:** reach all 8 checkpoint organs and return to `left_ventricle`. Keep your hull above 0% and your ATP above 0 — running dry leaves you stranded.
