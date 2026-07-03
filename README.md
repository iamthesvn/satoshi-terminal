# Satoshi's Terminal

> A **TUI (terminal user interface)** game for learning Bitcoin. Runs entirely in your terminal — no GUI, just text, color, and ASCII art.

```
 ██████╗  █████╗  ████████╗  ██████╗   ██████╗ ██╗  ██╗  ██╗  ██╗  ██████╗
██╔════╝ ██╔══██╗ ╚══██╔══╝ ██╔═══██╗ ██╔════╝ ██║  ██║  ██║  ██║ ██╔════╝
╚█████╗  ███████║    ██║    ██║   ██║ ╚█████╗  ███████║  ██║  ╚═╝ ╚█████╗
 ╚═══██║ ██╔══██║    ██║    ██║   ██║  ╚═══██║ ██╔══██║  ██║       ╚═══██║
██████╔╝ ██║  ██║    ██║    ╚██████╔╝ ██████╔╝ ██║  ██║  ██║      ██████╔╝
╚═════╝  ╚═╝  ╚═╝    ╚═╝     ╚═════╝  ╚═════╝  ╚═╝  ╚═╝  ╚═╝      ╚═════╝
████████╗ ████████╗ ██████╗   ███╗   ███╗ ██╗ ██╗   ██╗  █████╗  ██╗
╚══██╔══╝ ██╔═════╝ ██╔══██╗  ████╗ ████║ ██║ ███╗  ██║ ██╔══██╗ ██║
   ██║    █████╗    ██████╔╝  ██╔████╔██║ ██║ ██╔██╗██║ ███████║ ██║
   ██║    ██╔══╝    ██╔══██╗  ██║╚██╔╝██║ ██║ ██║╚████║ ██╔══██║ ██║
   ██║    ███████╗  ██║  ██║  ██║ ╚═╝ ██║ ██║ ██║ ╚███║ ██║  ██║ ███████╗
   ╚═╝    ╚══════╝  ╚═╝  ╚═╝  ╚═╝     ╚═╝ ╚═╝ ╚═╝  ╚══╝ ╚═╝  ╚═╝ ╚══════╝
```

You are **Alex**, a curious mind stepping into **Satoshi Labs**. From running your first
Bitcoin Core node to understanding Taproot and the Lightning Network, every chapter drops you
into a real protocol concept. Type the right answer or command and level up your Bitcoin IQ.

---

## Quick start

```bash
git clone https://github.com/iamthesvn/satoshi-terminal
cd satoshi-terminal
cargo install --path .
satoshi-terminal
```

Requires Rust stable (1.75+). No external dependencies — audio is synthesised at runtime, no files to bundle.

---

## The story — Satoshi Labs

Satoshi's Terminal is structured as multiple volumes covering the full Bitcoin protocol stack.
Each chapter drops you into a concept, asks you a question, or has you type a real `bitcoin-cli`
command.

### Volume 1 — Foundations
*"Before you stack sats, you must understand the protocol."*

| Ch | Title |
|----|-------|
| 1 | The Whitepaper |
| 2 | Hello Node |
| 3 | Block Count |
| 4 | Peer Connections |
| 5 | The Origin Block |
| 6 | Explorer |

### Volume 2 — Keys & Wallets
*"Not your keys, not your coins."*

| Ch | Title |
|----|-------|
| 7 | Private Key Secrets |
| 8 | Shareable Identifier |
| 9 | The Word List |
| 10 | Wallet Backup |
| 11 | Signature Separation |
| 12 | Native Address Encoding |

### Volume 3 — Transactions
*"Every transaction tells a story on the blockchain."*

| Ch | Title |
|----|-------|
| 13 | Unspent Outputs |
| 14 | Creating Raw TX |
| 15 | Signing |
| 16 | Broadcasting |
| 17 | Shared Control |
| 18 | Hidden Script Paths |
| 19 | Miner Incentive |
| 20 | Time-Based Restriction |
| 21 | Permanent Data Storage |

### Volume 4 — Network & Blockchain
*"The gears that turn every 10 minutes."*

| Ch | Title |
|----|-------|
| 22 | Transaction Queue |
| 23 | Proof of Work |
| 24 | Block Hash |
| 25 | Difficulty Adjustment |
| 26 | Block Summary Tree |
| 27 | Backward Compatible Upgrade |
| 28 | Light Client Verification |

### Volume 5 — Mining, Security & Layers
*"Dig deep, stay secure, and look beyond the base chain."*

| Ch | Title |
|----|-------|
| 29 | Block Reward Transaction |
| 30 | Subsidy Reduction |
| 31 | 21 Million |
| 32 | Off-Chain Payments |
| 33 | Not Your Keys |
| 34 | Collaborative Mining |
| 35 | 51% Attack |
| 36 | Air-Gapped Vault |

---

## The cast

| Character | Role | Personality |
|-----------|------|-------------|
| **Zoe** | Node operator / mentor | Patient, methodical, believes in running your own node |
| **Marcus** | Cryptography lead | Intense about key security, hates cloud backups |
| **Elena** | Protocol researcher | Explains scripts, signatures, and layer-2 scaling |
| **Dr. Hal** | Mining & consensus expert | Obsessed with difficulty, hashes, and economic incentives |

---

## Hint system

Every chapter has **3 tiered hints** — you decide how much help you want.

| Key | Action |
|-----|--------|
| `?` | Open / close the hint panel |
| `Tab` | Reveal the next hint tier (only when panel is open) |

Hints go from vague nudge → concept category → near-exact answer.
Each hint used costs a small Sats penalty. You always earn at least 25% of the base Sats.

---

## Controls

| Key | Action |
|-----|--------|
| `↑ ↓` / `j k` | Navigate menus |
| `Enter` | Confirm / submit answer |
| `Backspace` | Edit your answer |
| `?` | Toggle hint panel |
| `Tab` | Reveal next hint (panel must be open) |
| `M` | Mute / unmute background music |
| `Esc` | Back / main menu |
| `Ctrl+C` | Quit |

---

## Scoring

- Each chapter has a base Sats value depending on difficulty
- **-2 Sats** per extra attempt after the first
- **-3 Sats** per hint tier revealed
- Score floors at **25%** of base — you always earn something
- Answers are matched case-insensitively with collapsed whitespace, so minor formatting differences don't count as wrong

**Titles** (total Sats across all chapters):

| Sats | Title |
|------|-------|
| 0–49 | Curious Observer |
| 50–149 | Wallet Holder |
| 150–299 | Transaction Builder |
| 300–499 | Node Operator |
| 500–749 | Block Miner |
| 750–999 | Protocol Architect |
| 1000+ | Satoshi's Successor |

---

## Features

- Narrative-driven gameplay — real mentors, real protocol tension
- Multiple volumes of chapters covering Bitcoin from whitepaper to Lightning
- 3-tier hint system with Sats penalty — not hand-holding, just scaffolding
- Synthesised ambient music (3 Bitcoin-themed tracks) + sound effects via `rodio` — no audio files
- ₿ tile-flood transition animation between chapters
- Save / continue system at `~/.satoshi-terminal/save.json`
- Terminal minimum 80×24 — shows resize warning if too small
- Smooth UI animations via `vyfor/animate`
- Runs on macOS, Linux, and Windows

---

## Tech

| | |
|---|---|
| Language | Rust 2024 |
| TUI | [ratatui](https://github.com/ratatui/ratatui) 0.30 + crossterm 0.29 |
| Animations | [vyfor/animate](https://github.com/vyfor/animate) |
| Audio | rodio 0.19 (synthesised — no bundled files) |
| Save | serde_json + dirs |

---

## Adding a chapter

1. Add a new `Chapter { .. }` entry to the relevant volume in `src/volumes/story.rs`
2. Fill in: `title`, `scene_art`, `npc_name`, `npc_dialogue`, `task_prompt`, `accepted_answers`, `hints` (3 items), `success_message`, `xp`
3. That's it — the engine picks it up automatically

To add a whole new volume, append a new `Volume { .. }` to the `vec![]` in `all_volumes()`.

---

## License

MIT
