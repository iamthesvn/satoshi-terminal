# Satoshi's Terminal

> Learn Bitcoin by living it. A terminal game where every answer unlocks the protocol one block at a time.

```
  ██████╗ ████████╗  ██████╗ ██╗   ██╗███████╗███████╗████████╗
  ██╔══██╗╚══██╔══╝ ██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
  ██████╔╝   ██║   ██║   ██║██║   ██║█████╗  ███████╗   ██║
  ██╔══██╗   ██║   ██║   ██║██║   ██║██╔══╝  ╚════██║   ██║
  ██████╔╝   ██║   ╚██████╔╝╚██████╔╝███████╗███████║   ██║
   ╚═════╝    ╚═╝    ╚═════╝ ╚═════╝ ╚══════╝╚══════╝   ╚═╝
```

You are **Saylor**, a curious mind stepping into **Satoshi Labs**. From running your first
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

Satoshi's Terminal is structured as **5 volumes** covering the full Bitcoin protocol stack
(36 chapters total). Each chapter drops you into a concept, asks you a question, or
has you type a real `bitcoin-cli` command.

### Volume 1 — Foundations
*"Before you stack sats, you must understand the protocol."*

| Ch | Title | Answer / Command |
|----|-------|------------------|
| 1 | The Whitepaper | `Satoshi Nakamoto` |
| 2 | Hello Node | `bitcoin-cli getblockchaininfo` |
| 3 | Block Count | `bitcoin-cli getblockcount` |
| 4 | Peer Connections | `bitcoin-cli getconnectioncount` |
| 5 | Genesis Block | `genesis block` |
| 6 | Explorer | `blockchain explorer` |

### Volume 2 — Keys & Wallets
*"Not your keys, not your coins."*

| Ch | Title | Answer / Command |
|----|-------|------------------|
| 7 | Private Key Secrets | `private key` |
| 8 | Public Address | `address` |
| 9 | Mnemonic Seed | `seed phrase` |
| 10 | Wallet Backup | `bitcoin-cli backupwallet` |
| 11 | SegWit | `segwit` |
| 12 | Bech32 | `bech32` |

### Volume 3 — Transactions
*"Every transaction tells a story on the blockchain."*

| Ch | Title | Answer / Command |
|----|-------|------------------|
| 13 | UTXO | `UTXO` |
| 14 | Creating Raw TX | `bitcoin-cli createrawtransaction` |
| 15 | Signing | `bitcoin-cli signrawtransactionwithwallet` |
| 16 | Broadcasting | `bitcoin-cli sendrawtransaction` |
| 17 | Multisig | `multisig` |
| 18 | Taproot | `taproot` |
| 19 | Transaction Fees | `transaction fee` |
| 20 | Locktime | `locktime` |
| 21 | OP_RETURN | `OP_RETURN` |

### Volume 4 — Network & Blockchain
*"The gears that turn every 10 minutes."*

| Ch | Title | Answer / Command |
|----|-------|------------------|
| 22 | The Mempool | `mempool` |
| 23 | Proof of Work | `nonce` |
| 24 | Block Hash | `block hash` |
| 25 | Difficulty Adjustment | `2016` |
| 26 | Merkle Tree | `merkle tree` |
| 27 | Soft vs Hard Forks | `soft fork` |
| 28 | SPV | `simplified payment verification` |

### Volume 5 — Mining, Security & Layers
*"Dig deep, stay secure, and look beyond the base chain."*

| Ch | Title | Answer / Command |
|----|-------|------------------|
| 29 | Coinbase Transaction | `coinbase` |
| 30 | The Halving | `halving` |
| 31 | 21 Million | `21 million` |
| 32 | Lightning Network | `lightning network` |
| 33 | Not Your Keys | `coins` |
| 34 | Mining Pools | `mining pool` |
| 35 | 51% Attack | `51%` |
| 36 | Hardware Wallet | `hardware wallet` |

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
Each hint used costs a small XP penalty. You always earn at least 25% of the base XP.

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

- Each chapter has a base XP value (10–35 XP depending on difficulty)
- **-2 XP** per extra attempt after the first
- **-3 XP** per hint tier revealed
- Score floors at **25%** of base — you always earn something
- Answers are matched case-insensitively with collapsed whitespace, so minor formatting differences don't count as wrong

**Ranks** (total XP across all 36 chapters):

| XP | Rank |
|----|------|
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
- **5 volumes × 36 chapters** covering Bitcoin from whitepaper to Lightning
- 3-tier hint system with XP penalty — not hand-holding, just scaffolding
- Synthesised ambient music (3 Bitcoin-themed tracks) + sound effects via `rodio` — no audio files
- Bitcoin-coin flood transition animation between chapters
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
