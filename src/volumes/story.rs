// volumes/story.rs — Satoshi's Terminal story content and chapter data
// Satoshi Labs saga: Saylor's journey from Bitcoin curious to node operator.

#[derive(Clone)]
pub struct Chapter {
    pub title: &'static str,
    pub scene_art: &'static [&'static str],
    pub npc_name: &'static str,
    pub npc_dialogue: &'static [&'static str],
    pub task_prompt: &'static str,
    pub accepted_answers: &'static [&'static str],
    pub hints: &'static [&'static str],
    pub success_message: &'static str,
    pub xp: u32,
    /// Time limit in seconds for this chapter.
    pub time_limit_secs: u32,
}

#[derive(Clone)]
pub struct Volume {
    pub id: usize,
    pub title: &'static str,
    pub tagline: &'static str,
    pub chapters: Vec<Chapter>,
}

// ---------------------------------------------------------------------------
// RANK TITLES
// ---------------------------------------------------------------------------

pub fn rank_title(xp: u32) -> &'static str {
    match xp {
        0..=49 => "Curious Observer",
        50..=149 => "Wallet Holder",
        150..=299 => "Transaction Builder",
        300..=499 => "Node Operator",
        500..=749 => "Block Miner",
        750..=999 => "Protocol Architect",
        _ => "Satoshi's Successor",
    }
}

// ---------------------------------------------------------------------------
// ALL VOLUMES
// ---------------------------------------------------------------------------

pub fn all_volumes() -> Vec<Volume> {
    vec![volume_1(), volume_2(), volume_3(), volume_4(), volume_5()]
}

// ==========================================================================
// VOLUME 1 — Foundations
// ==========================================================================

fn volume_1() -> Volume {
    Volume {
        id: 1,
        title: "Foundations",
        tagline: "Before you stack sats, you must understand the protocol.",
        chapters: vec![
            // ---------------------------------------------------------------
            // Chapter 1 — The Whitepaper
            // ---------------------------------------------------------------
            Chapter {
                title: "The Whitepaper",
                scene_art: &[
                    r"    ┌──────────────────────┐     ",
                    r"    │  Bitcoin: A Peer-to  │     ",
                    r"    │  Peer Electronic     │     ",
                    r"    │  Cash System         │     ",
                    r"    │  by S. Nakamoto      │     ",
                    r"    │         [2008]       │     ",
                    r"    └──────────────────────┘     ",
                    r"         📜  scroll              ",
                ],
                npc_name: "Zoe",
                npc_dialogue: &[
                    "Welcome to Satoshi Labs, Alex. I'm Zoe — I'll be your node-running mentor.",
                    "Before you touch a single satoshi, you need to know where this all began.",
                    "In 2008, a pseudonymous author published a paper that changed money forever.",
                    "What name did that author use?",
                ],
                task_prompt: "Enter the pseudonymous creator of Bitcoin.",
                accepted_answers: &["satoshi nakamoto", "satoshi", "Satoshi Nakamoto", "Satoshi"],
                hints: &[
                    "A pseudonymous author published the Bitcoin whitepaper in 2008.",
                    "Two words: first name Satoshi, last name Nakamoto.",
                ],
                success_message: "Correct. The whitepaper laid the foundation for everything we do here.",
                xp: 10,
                time_limit_secs: 60,
            },
            // ---------------------------------------------------------------
            // Chapter 2 — Hello Node
            // ---------------------------------------------------------------
            Chapter {
                title: "Hello Node",
                scene_art: &[
                    r"      ┌─────────────┐            ",
                    r"      │  BITCOIN    │            ",
                    r"      │    CORE     │            ",
                    r"      │  ════════   │            ",
                    r"      │  [SYNCED]   │            ",
                    r"      └─────────────┘            ",
                    r"         │ │ │                   ",
                    r"    🖥️  server rack               ",
                ],
                npc_name: "Zoe",
                npc_dialogue: &[
                    "Good. Now let's get our hands dirty.",
                    "We've got a Bitcoin Core node running on this machine.",
                    "The first thing every operator does is check the health of the blockchain.",
                    "Use bitcoin-cli to fetch the blockchain info.",
                ],
                task_prompt: "Query the node's blockchain information via RPC.",
                accepted_answers: &["bitcoin-cli getblockchaininfo"],
                hints: &[
                    "The command starts with bitcoin-cli.",
                    "You want to 'get' information about the 'blockchain'.",
                    "Try: bitcoin-cli getblockchaininfo",
                ],
                success_message: "Node is synced and healthy. You can see the chain height and network info.",
                xp: 10,
                time_limit_secs: 60,
            },
            // ---------------------------------------------------------------
            // Chapter 3 — Block Count
            // ---------------------------------------------------------------
            Chapter {
                title: "Block Count",
                scene_art: &[
                    r"       [ BLOCK ]                 ",
                    r"      ┌───────┐                  ",
                    r"      │ 840k+ │                  ",
                    r"      └───┬───┘                  ",
                    r"      ┌───┴───┐                  ",
                    r"      │ 839k+ │                  ",
                    r"      └───┬───┘                  ",
                    r"    🧱 chain grows downward       ",
                ],
                npc_name: "Zoe",
                npc_dialogue: &[
                    "Blocks are the heartbeat of Bitcoin.",
                    "Every ~10 minutes, a new block is mined and the chain grows by one.",
                    "Let's check how tall our local copy of the blockchain is.",
                ],
                task_prompt: "Get the current block height from the node.",
                accepted_answers: &["bitcoin-cli getblockcount"],
                hints: &[
                    "You want the numeric count of blocks.",
                    "Try: bitcoin-cli getblockcount",
                ],
                success_message: "Block count retrieved. The chain keeps growing, one block at a time.",
                xp: 10,
                time_limit_secs: 60,
            },
            // ---------------------------------------------------------------
            // Chapter 4 — Peer Connections
            // ---------------------------------------------------------------
            Chapter {
                title: "Peer Connections",
                scene_art: &[
                    r"         ●───●                   ",
                    r"        /│\  /│\                  ",
                    r"       ●─●─●─●─●─●                ",
                    r"        │\ / │ /                  ",
                    r"         ●───●                   ",
                    r"         peer mesh               ",
                    r"                                 ",
                    r"    🌐 no central server         ",
                ],
                npc_name: "Zoe",
                npc_dialogue: &[
                    "Bitcoin has no central server. It survives because thousands of nodes talk to each other.",
                    "Your node needs peers to receive blocks and transactions.",
                    "Let's see how many connections you currently have.",
                ],
                task_prompt: "Check how many peer connections the node has.",
                accepted_answers: &["bitcoin-cli getconnectioncount"],
                hints: &[
                    "You want to count the connections.",
                    "Try: bitcoin-cli getconnectioncount",
                ],
                success_message: "Connections confirmed. You're part of the network now.",
                xp: 10,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 5 — Genesis Block
            // ---------------------------------------------------------------
            Chapter {
                title: "The Origin Block",
                scene_art: &[
                    r"      ┌─────────────┐            ",
                    r"      │  BLOCK 0    │            ",
                    r"      │  2009-01-03 │            ",
                    r"      │  The Times  │            ",
                    r"      │  03/Jan/09  │            ",
                    r"      └─────────────┘            ",
                    r"         🏛️ origin               ",
                ],
                npc_name: "Zoe",
                npc_dialogue: &[
                    "Every blockchain starts with a single block that has no predecessor.",
                    "Satoshi embedded a newspaper headline in it to prove the launch date.",
                    "What do we call this very first block?",
                ],
                task_prompt: "Enter the term for the first block in any blockchain.",
                accepted_answers: &["genesis block", "Genesis Block"],
                hints: &[
                    "The first book of the Bible shares this name.",
                    "It has no previous block hash and marks the very beginning of the chain.",
                ],
                success_message: "Genesis Block — the big bang of the Bitcoin blockchain.",
                xp: 15,
                time_limit_secs: 45,
            },
            // ---------------------------------------------------------------
            // Chapter 6 — Explorer
            // ---------------------------------------------------------------
            Chapter {
                title: "Explorer",
                scene_art: &[
                    r"      ┌─────────────────────┐    ",
                    r"      │ 🔍 search block/tx  │    ",
                    r"      │ 00000...            │    ",
                    r"      │ [ lookup history ]  │    ",
                    r"      └─────────────────────┘    ",
                    r"      mempool.space / blockstream ",
                    r"                                 ",
                    r"    🔎 blockchain search engine   ",
                ],
                npc_name: "Zoe",
                npc_dialogue: &[
                    "To understand Bitcoin, you need to read the ledger.",
                    "Web tools let you inspect blocks, transactions, and addresses in real time.",
                    "What do we call these tools?",
                ],
                task_prompt: "Enter the term for a web tool that lets you inspect the blockchain.",
                accepted_answers: &["blockchain explorer", "block explorer", "explorer"],
                hints: &[
                    "It's like Google, but for blocks and transactions.",
                    "These web tools let you look up transactions, addresses, and blocks by hash or height.",
                ],
                success_message: "Explorer — the window into Bitcoin's transparent ledger.",
                xp: 15,
                time_limit_secs: 30,
            },
        ],
    }
}

// ==========================================================================
// VOLUME 2 — Keys & Wallets
// ==========================================================================

fn volume_2() -> Volume {
    Volume {
        id: 2,
        title: "Keys & Wallets",
        tagline: "Not your keys, not your coins.",
        chapters: vec![
            // ---------------------------------------------------------------
            // Chapter 7 — Private Key Secrets
            // ---------------------------------------------------------------
            Chapter {
                title: "Private Key Secrets",
                scene_art: &[
                    r"      ┌─────────────┐            ",
                    r"      │ 🔒  LOCKED  │            ",
                    r"      │   VAULT    │            ",
                    r"      └──────┬──────┘            ",
                    r"            🔑                   ",
                    r"      secret key only            ",
                    r"                                 ",
                    r"    🗝️  guard with your life      ",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "I'm Marcus. I handle cryptography here.",
                    "In Bitcoin, ownership isn't about having an account password.",
                    "It's about knowing a secret number that only you should possess.",
                    "What do we call that secret number?",
                ],
                task_prompt: "Name the secret number that proves ownership of bitcoin.",
                accepted_answers: &["private key", "privkey", "secret key"],
                hints: &[
                    "It's the opposite of a public key.",
                    "This 256-bit secret controls all funds in your wallet. Never share it.",
                ],
                success_message: "Exactly. Guard it with your life. Lose the key, lose the coins.",
                xp: 15,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 8 — Public Address
            // ---------------------------------------------------------------
            Chapter {
                title: "Shareable Identifier",
                scene_art: &[
                    r"      ┌───────────────┐          ",
                    r"      │ bc1qxy2k...   │          ",
                    r"      │   [ SHARE ]   │          ",
                    r"      └───────────────┘          ",
                    r"           📨                    ",
                    r"     safe to broadcast           ",
                    r"                                 ",
                    r"    📮 receive address           ",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "From your private key, we derive a public key.",
                    "From that, we generate a human-readable string that others use to pay you.",
                    "What do we call that receiving string?",
                ],
                task_prompt: "Enter the term for a shareable string used to receive bitcoin.",
                accepted_answers: &["address", "bitcoin address", "public address"],
                hints: &[
                    "It's like an email address, but for money.",
                    "It's derived from your public key and can be shared safely with anyone.",
                ],
                success_message: "Correct. You can share your address freely. Just never share the private key.",
                xp: 15,
                time_limit_secs: 45,
            },
            // ---------------------------------------------------------------
            // Chapter 9 — Mnemonic Seed
            // ---------------------------------------------------------------
            Chapter {
                title: "The Word List",
                scene_art: &[
                    r"   ┌──────────────────────┐      ",
                    r"   │ 1. abandon           │      ",
                    r"   │ 2. ability           │      ",
                    r"   │ 3. able              │      ",
                    r"   │ ... (12/24 words)    │      ",
                    r"   │ [ keep offline ]     │      ",
                    r"   └──────────────────────┘      ",
                    r"       📝 seed phrase             ",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "Modern wallets don't make you back up a raw private key.",
                    "Instead, they give you a list of words — usually 12 or 24 — that encode all your keys.",
                    "What do we call that word list?",
                ],
                task_prompt: "Enter the term for the word list that backs up a modern wallet.",
                accepted_answers: &[
                    "mnemonic",
                    "seed phrase",
                    "mnemonic seed",
                    "recovery phrase",
                ],
                hints: &[
                    "It helps you remember your wallet.",
                    "BIP-39 standardised this 12- or 24-word encoding for wallet backups.",
                ],
                success_message: "Right. Write it down. Keep it offline. Never store it in the cloud.",
                xp: 15,
                time_limit_secs: 60,
            },
            // ---------------------------------------------------------------
            // Chapter 10 — Wallet Backup
            // ---------------------------------------------------------------
            Chapter {
                title: "Wallet Backup",
                scene_art: &[
                    r"      💾 wallet.dat              ",
                    r"      ┌─────────────┐            ",
                    r"      │  BACKUP    │            ",
                    r"      │  COPY      │            ",
                    r"      │  >>>>      │            ",
                    r"      └─────────────┘            ",
                    r"       to external drive         ",
                    r"                                 ",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "Sometimes you need to back up the actual wallet file programmatically.",
                    "Bitcoin Core has an RPC command that creates a copy of the wallet to a destination path.",
                    "What's the command?",
                ],
                task_prompt: "Type the bitcoin-cli command to back up the wallet.",
                accepted_answers: &["bitcoin-cli backupwallet"],
                hints: &[
                    "The command has two parts: what you want to do, and where.",
                    "Try: bitcoin-cli backupwallet",
                ],
                success_message: "Backup created. Redundancy is the only insurance in Bitcoin.",
                xp: 15,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 11 — SegWit
            // ---------------------------------------------------------------
            Chapter {
                title: "Signature Separation",
                scene_art: &[
                    r"      ┌───────────────┐          ",
                    r"      │   tx data     │          ",
                    r"      │ ───────────── │          ",
                    r"      │  witness 🔏   │          ",
                    r"      │  [separated]  │          ",
                    r"      └───────────────┘          ",
                    r"      malleability fix           ",
                    r"                                 ",
                    r"    📦 signature segregation      ",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "A major protocol upgrade moved signature data outside the base transaction structure.",
                    "This fixed transaction malleability and allowed more transactions per block.",
                    "What is this upgrade called?",
                ],
                task_prompt: "Name the upgrade that separates witness data from transaction data.",
                accepted_answers: &[
                    "segwit",
                    "SegWit",
                    "Segregated Witness",
                    "segregated witness",
                ],
                hints: &[
                    "Short for Segregated Witness.",
                    "By moving witness data outside the base structure, this fixed transaction malleability.",
                ],
                success_message: "SegWit — separating signatures to scale Bitcoin securely.",
                xp: 20,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 12 — Bech32
            // ---------------------------------------------------------------
            Chapter {
                title: "Native Address Encoding",
                scene_art: &[
                    r"      ┌───────────────┐          ",
                    r"      │ bc1qxy2k...   │          ",
                    r"      │  lower case   │          ",
                    r"      │  no mixed     │          ",
                    r"      │  [native sw]  │          ",
                    r"      └───────────────┘          ",
                    r"      starts with bc1            ",
                    r"                                 ",
                    r"    📮 native segwit format       ",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "Native SegWit addresses use a special encoding format.",
                    "They are all lowercase, start with 'bc1', and have better error detection.",
                    "What do we call this address format?",
                ],
                task_prompt: "Enter the name of the address encoding used for native SegWit addresses.",
                accepted_answers: &["bech32", "Bech32"],
                hints: &[
                    "Sounds like a command to a dog: 'beach, 32!'",
                    "Native SegWit addresses use this all-lowercase encoding with better error detection.",
                ],
                success_message: "Bech32 — the cleaner, lighter address format for native SegWit.",
                xp: 20,
                time_limit_secs: 30,
            },
        ],
    }
}

// ==========================================================================
// VOLUME 3 — Transactions
// ==========================================================================

fn volume_3() -> Volume {
    Volume {
        id: 3,
        title: "Transactions",
        tagline: "Every transaction tells a story on the blockchain.",
        chapters: vec![
            // ---------------------------------------------------------------
            // Chapter 13 — UTXO
            // ---------------------------------------------------------------
            Chapter {
                title: "Unspent Outputs",
                scene_art: &[
                    r"      ┌──────────────┐            ",
                    r"      │  2.5 BTC 💰  │            ",
                    r"      │   UNSPENT    │            ",
                    r"      └──────┬───────┘            ",
                    r"             │                   ",
                    r"      [TX_IN]──►[TX_OUT]         ",
                    r"                                 ",
                    r"     🪙 spendable chunk           ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "I'm Elena. I study transactions and scripts.",
                    "Bitcoin doesn't use accounts with balances. Instead, it tracks spendable chunks of value.",
                    "What do we call an output that hasn't been spent yet?",
                ],
                task_prompt: "Enter the acronym for an unspent transaction output.",
                accepted_answers: &["utxo", "UTXO"],
                hints: &[
                    "Unspent Transaction Output.",
                    "Bitcoin doesn't track account balances; it tracks individual spendable chunks of value.",
                ],
                success_message: "UTXO — the fundamental unit of value in Bitcoin. Master this and you master the ledger.",
                xp: 15,
                time_limit_secs: 60,
            },
            // ---------------------------------------------------------------
            // Chapter 14 — Creating Raw TX
            // ---------------------------------------------------------------
            Chapter {
                title: "Creating Raw TX",
                scene_art: &[
                    r"      ┌─────────────────────┐     ",
                    r"      │ 0100000001abcdef... │     ",
                    r"      │ [  unsigned  hex  ] │     ",
                    r"      └─────────────────────┘     ",
                    r"      txid:vout ──► outputs       ",
                    r"                                 ",
                    r"    🔧 assemble transaction        ",
                    r"                                 ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "Before a transaction can be broadcast, it must be constructed.",
                    "Bitcoin Core lets you build a raw, unsigned transaction from inputs and outputs.",
                    "What's the RPC command to create one?",
                ],
                task_prompt: "Type the bitcoin-cli command to create a raw transaction.",
                accepted_answers: &["bitcoin-cli createrawtransaction"],
                hints: &[
                    "You want to create a raw transaction.",
                    "You need to specify inputs (txid:vout) and outputs (address:amount) in JSON format.",
                ],
                success_message: "Raw transaction hex generated. Next step: sign it.",
                xp: 15,
                time_limit_secs: 75,
            },
            // ---------------------------------------------------------------
            // Chapter 15 — Signing
            // ---------------------------------------------------------------
            Chapter {
                title: "Signing",
                scene_art: &[
                    r"      🔑  +  📄                   ",
                    r"         │                        ",
                    r"         ▼                        ",
                    r"      ┌───────────────┐           ",
                    r"      │  SIGNED TX   │           ",
                    r"      │  ✓ VALID    │           ",
                    r"      └───────────────┘           ",
                    r"    ✍️  cryptographic proof       ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "A raw transaction is useless until it's signed.",
                    "Signing proves you own the inputs and authorizes the spend.",
                    "Use bitcoin-cli to sign a raw transaction with the wallet's keys.",
                ],
                task_prompt: "Type the bitcoin-cli command to sign a raw transaction with the wallet.",
                accepted_answers: &["bitcoin-cli signrawtransactionwithwallet"],
                hints: &[
                    "The command signs a raw transaction.",
                    "The wallet uses your private keys to produce a valid signature for the unsigned hex.",
                ],
                success_message: "Transaction signed. The signature cryptographically proves ownership.",
                xp: 15,
                time_limit_secs: 60,
            },
            // ---------------------------------------------------------------
            // Chapter 16 — Broadcasting
            // ---------------------------------------------------------------
            Chapter {
                title: "Broadcasting",
                scene_art: &[
                    r"         📡                       ",
                    r"        / │ \                     ",
                    r"       /  │  \                    ",
                    r"      ●───●───●                   ",
                    r"      peers around world           ",
                    r"                                 ",
                    r"    📻 mempool gossip             ",
                    r"                                 ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "Now that it's signed, the transaction needs to reach the network.",
                    "Bitcoin Core can broadcast a signed raw transaction to all connected peers.",
                    "What's the command?",
                ],
                task_prompt: "Type the bitcoin-cli command to broadcast a raw transaction.",
                accepted_answers: &["bitcoin-cli sendrawtransaction"],
                hints: &[
                    "You want to send the raw transaction to the network.",
                    "Once signed, the raw hex can be propagated to the peer-to-peer network.",
                ],
                success_message: "Broadcast successful. The transaction is now in mempools around the world.",
                xp: 15,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 17 — Multisig
            // ---------------------------------------------------------------
            Chapter {
                title: "Shared Control",
                scene_art: &[
                    r"      🔑   🔑                     ",
                    r"       \  /                       ",
                    r"        🔒                        ",
                    r"       /                          ",
                    r"      🔑                          ",
                    r"      2-of-3 required             ",
                    r"                                 ",
                    r"    🔐 shared control             ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "Sometimes one signature isn't enough.",
                    "Businesses and protocols require multiple parties to agree before funds move.",
                    "What's the short name for a script that requires M-of-N signatures?",
                ],
                task_prompt: "Enter the term for M-of-N signature requirements.",
                accepted_answers: &["multisig", "multisignature", "multi-sig"],
                hints: &[
                    "Short for multiple signatures.",
                    "This script requires M valid signatures out of N possible public keys.",
                ],
                success_message: "Multisig understood. Shared control is shared security.",
                xp: 15,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 18 — Taproot
            // ---------------------------------------------------------------
            Chapter {
                title: "Hidden Script Paths",
                scene_art: &[
                    r"         🌳                       ",
                    r"        /│\                       ",
                    r"       / │ \                      ",
                    r"      🍃  │  🍃                    ",
                    r"         │                        ",
                    r"    hidden script branches        ",
                    r"    look like single-key          ",
                    r"    🌱 privacy upgrade            ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "Bitcoin keeps evolving. One of the most important upgrades improved privacy and smart contracts.",
                    "It lets complex scripts look like simple single-key spends on the blockchain.",
                    "What's the name of this upgrade?",
                ],
                task_prompt: "Name the Bitcoin upgrade that improves privacy and smart contracts.",
                accepted_answers: &["taproot", "Taproot"],
                hints: &[
                    "It's named after a plant root system.",
                    "This upgrade uses Schnorr signatures and Merkelised alternative script trees.",
                ],
                success_message: "Taproot — where all spend paths look the same. Privacy by design.",
                xp: 20,
                time_limit_secs: 45,
            },
            // ---------------------------------------------------------------
            // Chapter 19 — Transaction Fees
            // ---------------------------------------------------------------
            Chapter {
                title: "Miner Incentive",
                scene_art: &[
                    r"      ┌─────────────────┐          ",
                    r"      │  inputs         │          ",
                    r"      │    ─ output ─   │          ",
                    r"      │  = fee to miner │          ",
                    r"      │  (sat/vB)       │          ",
                    r"      └─────────────────┘          ",
                    r"      🔥 unspent difference        ",
                    r"                                 ",
                    r"    💸 incentive for miners       ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "Every transaction must include an incentive for miners to include it in a block.",
                    "It's the difference between inputs and outputs, measured in satoshis per virtual byte.",
                    "What do we call this payment to miners?",
                ],
                task_prompt: "Enter the term for the payment miners receive from transactions.",
                accepted_answers: &["transaction fee", "fee", "miner fee", "tx fee"],
                hints: &[
                    "You pay this to get your transaction confirmed faster.",
                    "Miners prioritise transactions offering higher satoshis per virtual byte.",
                ],
                success_message: "Transaction fee — the market price for block space.",
                xp: 20,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 20 — Locktime
            // ---------------------------------------------------------------
            Chapter {
                title: "Time-Based Restriction",
                scene_art: &[
                    r"      ┌─────────────┐            ",
                    r"      │  ⏰ LOCKED  │            ",
                    r"      │  until      │            ",
                    r"      │  block 1M   │            ",
                    r"      │  or 2026-01 │            ",
                    r"      └─────────────┘            ",
                    r"      time-based restriction     ",
                    r"                                 ",
                    r"    ⏳ future spendable          ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "Bitcoin transactions can include a rule that prevents spending until a certain time or block height.",
                    "This is useful for time-locked contracts and scheduled payments.",
                    "What's the term for this time-based restriction?",
                ],
                task_prompt: "Enter the term for a transaction field that sets a spending time limit.",
                accepted_answers: &["locktime", "nlocktime", "lock time"],
                hints: &[
                    "It locks the transaction until a condition is met.",
                    "This field can specify either a block height or a Unix timestamp.",
                ],
                success_message: "Locktime — time travel for your bitcoin, but only forward.",
                xp: 20,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 21 — OP_RETURN
            // ---------------------------------------------------------------
            Chapter {
                title: "Permanent Data Storage",
                scene_art: &[
                    r"      ┌─────────────────────┐     ",
                    r"      │ 0x6a                │     ",
                    r"      │  [80 bytes max]     │     ",
                    r"      │  'hello world'      │     ",
                    r"      │  [provably unspend] │     ",
                    r"      └─────────────────────┘     ",
                    r"      📝 data in blockchain       ",
                    r"                                 ",
                    r"    📜 immutable message          ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "Bitcoin allows you to embed a small piece of data in a transaction output.",
                    "This output can never be spent, but it lives forever on the blockchain.",
                    "What opcode is used to create such a data-carrying output?",
                ],
                task_prompt: "Enter the opcode used to embed unspendable data in a transaction.",
                accepted_answers: &["op_return", "OP_RETURN"],
                hints: &[
                    "It's an operation that returns immediately, making the output unspendable.",
                    "This opcode immediately terminates script execution, making the output provably unspendable.",
                ],
                success_message: "OP_RETURN — Bitcoin's permanent notepad, limited to 80 bytes.",
                xp: 20,
                time_limit_secs: 30,
            },
        ],
    }
}

// ==========================================================================
// VOLUME 4 — Network & Blockchain
// ==========================================================================

fn volume_4() -> Volume {
    Volume {
        id: 4,
        title: "Network & Blockchain",
        tagline: "The gears that turn every 10 minutes.",
        chapters: vec![
            // ---------------------------------------------------------------
            // Chapter 22 — The Mempool
            // ---------------------------------------------------------------
            Chapter {
                title: "Transaction Queue",
                scene_art: &[
                    r"   ┌─────────────────────────┐    ",
                    r"   │  tx1 tx2 tx3 tx4 tx5   │    ",
                    r"   │  waiting...            │    ",
                    r"   │  fee: 12 sats/vB       │    ",
                    r"   │  fee: 8  sats/vB       │    ",
                    r"   │  fee: 4  sats/vB       │    ",
                    r"   └─────────────────────────┘    ",
                    r"    🏟️  transaction queue          ",
                ],
                npc_name: "Zoe",
                npc_dialogue: &[
                    "Transactions don't go straight into blocks.",
                    "They first wait in a temporary holding area on every node.",
                    "What do we call this holding area?",
                ],
                task_prompt: "Enter the term for the temporary pool of unconfirmed transactions.",
                accepted_answers: &["mempool", "memory pool"],
                hints: &[
                    "Short for memory pool.",
                    "Every node maintains its own temporary holding area for unconfirmed transactions.",
                ],
                success_message: "Mempool — the waiting room of Bitcoin. Transactions compete for block space here.",
                xp: 15,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 23 — Proof of Work
            // ---------------------------------------------------------------
            Chapter {
                title: "Proof of Work",
                scene_art: &[
                    r"      ⛏️                        ",
                    r"      │\                        ",
                    r"      │  \  hash < target        ",
                    r"      │   \  nonce++             ",
                    r"      🧱  🧱  🧱                 ",
                    r"      00000...                   ",
                    r"      leading zeros              ",
                    r"    ⚡ energy → security         ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "I'm Dr. Hal. I study consensus and mining.",
                    "Miners race to find a number that makes the block hash fall below a target.",
                    "What do we call that adjustable number in the block header?",
                ],
                task_prompt: "Enter the term for the number miners adjust to find a valid block hash.",
                accepted_answers: &["nonce", "Nonce"],
                hints: &[
                    "It's a 32-bit integer used only once.",
                    "Short for 'number used once'.",
                ],
                success_message: "Nonce found. Proof of work transforms electricity into immutable history.",
                xp: 20,
                time_limit_secs: 45,
            },
            // ---------------------------------------------------------------
            // Chapter 24 — Block Hash
            // ---------------------------------------------------------------
            Chapter {
                title: "Block Hash",
                scene_art: &[
                    r"      ┌────────────────────┐     ",
                    r"      │ 0000000000000a3f   │     ",
                    r"      │ ...fingerprint...  │     ",
                    r"      │ [ double SHA-256 ] │     ",
                    r"      └────────────────────┘     ",
                    r"            │                     ",
                    r"      🧱────┴────🧱               ",
                    r"    chain linkage                ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "Every block has a unique fingerprint derived from its header.",
                    "This fingerprint links blocks together into a chain.",
                    "What do we call this unique identifier?",
                ],
                task_prompt: "Enter the term for the unique hash identifier of a block.",
                accepted_answers: &["block hash", "hash", "blockhash"],
                hints: &[
                    "It's the result of hashing the block header.",
                    "This 64-character hex string uniquely identifies a block and is included in the next block's header.",
                ],
                success_message: "Block hash — the DNA of every block. Change one bit and the hash changes completely.",
                xp: 15,
                time_limit_secs: 20,
            },
            // ---------------------------------------------------------------
            // Chapter 25 — Difficulty Adjustment
            // ---------------------------------------------------------------
            Chapter {
                title: "Difficulty Adjustment",
                scene_art: &[
                    r"      ┌─────────────┐            ",
                    r"      │  2,016      │            ",
                    r"      │  blocks     │            ",
                    r"      │  ≈ 2 weeks  │            ",
                    r"      └──────┬──────┘            ",
                    r"             │                   ",
                    r"      🕰️  ~10 min target         ",
                    r"    protocol self-regulates      ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "Bitcoin aims for one block every 10 minutes on average.",
                    "To keep that schedule, the network recalculates mining difficulty every 2,016 blocks.",
                    "How many blocks is that? Just the number.",
                ],
                task_prompt: "Enter the number of blocks between difficulty adjustments.",
                accepted_answers: &["2016", "2,016"],
                hints: &[
                    "Approximately two weeks worth of blocks.",
                    "At 10 minutes per block, this is approximately two weeks of blocks.",
                ],
                success_message: "2016 blocks. The protocol self-regulates, no central planner needed.",
                xp: 20,
                time_limit_secs: 45,
            },
            // ---------------------------------------------------------------
            // Chapter 26 — Merkle Tree
            // ---------------------------------------------------------------
            Chapter {
                title: "Block Summary Tree",
                scene_art: &[
                    r"              🔝                 ",
                    r"            /  \                ",
                    r"          🔒    🔒               ",
                    r"         / \   / \              ",
                    r"        tx tx tx tx             ",
                    r"      root hash in header       ",
                    r"                                 ",
                    r"    🌳 cryptographic tree        ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "All transactions in a block are hashed together into a tree structure.",
                    "This lets you prove a transaction exists without downloading the whole block.",
                    "What do we call this tree of hashes?",
                ],
                task_prompt: "Enter the term for the tree of hashes that summarizes a block's transactions.",
                accepted_answers: &["merkle tree", "Merkle Tree", "merkle root"],
                hints: &[
                    "Named after Ralph Merkle.",
                    "Transactions are repeatedly paired and hashed until only a single root hash remains.",
                ],
                success_message: "Merkle tree — efficient cryptographic proof that a transaction is in a block.",
                xp: 20,
                time_limit_secs: 45,
            },
            // ---------------------------------------------------------------
            // Chapter 27 — Soft vs Hard Forks
            // ---------------------------------------------------------------
            Chapter {
                title: "Backward Compatible Upgrade",
                scene_art: &[
                    r"        🍴  FORK                ",
                    r"       /                      ",
                    r"    old rules  new rules      ",
                    r"    ┌─────┐   ┌─────┐         ",
                    r"    │valid│   │valid│         ",
                    r"    │only │   │both │         ",
                    r"    └─────┘   └─────┘         ",
                    r"    backward compatible?       ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "Bitcoin upgrades can change the rules. Some are backward compatible; some split the chain.",
                    "SegWit was one type. The 2017 block-size debate almost caused the other.",
                    "Which type of fork is backward compatible?",
                ],
                task_prompt: "Enter the type of fork that is backward compatible.",
                accepted_answers: &["soft fork", "Soft Fork"],
                hints: &[
                    "Old nodes still see new blocks as valid.",
                    "Old nodes still accept new blocks because the rules were tightened, not relaxed.",
                ],
                success_message: "Soft fork — tightening the rules without breaking old software.",
                xp: 20,
                time_limit_secs: 60,
            },
            // ---------------------------------------------------------------
            // Chapter 28 — SPV
            // ---------------------------------------------------------------
            Chapter {
                title: "Light Client Verification",
                scene_art: &[
                    r"      📱 mobile wallet           ",
                    r"         │                       ",
                    r"      ┌──┴──┐                   ",
                    r"      │headers│ only             ",
                    r"      │merkle│ proof             ",
                    r"      └─────┘                    ",
                    r"      no full blockchain         ",
                    r"    🔍 light verification        ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "Not everyone can run a full node with the entire blockchain.",
                    "Light wallets download only block headers and use Merkle proofs to verify payments.",
                    "What does the acronym SPV stand for?",
                ],
                task_prompt: "Enter what SPV stands for in Bitcoin.",
                accepted_answers: &[
                    "simplified payment verification",
                    "Simplified Payment Verification",
                ],
                hints: &[
                    "Light clients verify transactions using block headers and Merkle proofs, not full blocks.",
                    "SPV lets light clients verify without full history.",
                ],
                success_message: "SPV — trust minimization for devices that can't store 600+ GB.",
                xp: 20,
                time_limit_secs: 30,
            },
        ],
    }
}

// ==========================================================================
// VOLUME 5 — Mining, Security & Layers
// ==========================================================================

fn volume_5() -> Volume {
    Volume {
        id: 5,
        title: "Mining, Security & Layers",
        tagline: "Dig deep, stay secure, and look beyond the base chain.",
        chapters: vec![
            // ---------------------------------------------------------------
            // Chapter 29 — Coinbase Transaction
            // ---------------------------------------------------------------
            Chapter {
                title: "Block Reward Transaction",
                scene_art: &[
                    r"      ┌─────────────┐            ",
                    r"      │ BLOCK REWARD│            ",
                    r"      │  + fees     │            ",
                    r"      └──────┬──────┘            ",
                    r"             │                   ",
                    r"      🪙 3.125 BTC ──► ⛏️         ",
                    r"      no inputs, only output     ",
                    r"    genesis of new sats          ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "Every valid block contains a special transaction that pays the miner.",
                    "It has no inputs — only outputs — and it creates new bitcoin.",
                    "What's the name of this special transaction?",
                ],
                task_prompt: "Enter the name of the transaction that pays the miner and creates new bitcoin.",
                accepted_answers: &["coinbase", "coinbase transaction"],
                hints: &[
                    "Named after a famous cryptocurrency exchange, but it predates it.",
                    "This transaction has no inputs and creates new bitcoin as the miner's reward.",
                ],
                success_message: "Coinbase transaction — where all new bitcoin is born, one block at a time.",
                xp: 20,
                time_limit_secs: 30,
            },
            // ---------------------------------------------------------------
            // Chapter 30 — The Halving
            // ---------------------------------------------------------------
            Chapter {
                title: "Subsidy Reduction",
                scene_art: &[
                    r"      ┌───────────────┐          ",
                    r"      │  50 → 25 BTC  │          ",
                    r"      │ 25 → 12.5 BTC │          ",
                    r"      │12.5→ 6.25 BTC │          ",
                    r"      │ 3.125→...     │          ",
                    r"      └───────────────┘          ",
                    r"       every 210,000 blocks       ",
                    r"    📉 disinflationary clock      ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "Bitcoin has a fixed monetary policy. The block subsidy gets cut in half periodically.",
                    "This event happens every 210,000 blocks.",
                    "But what is this event commonly called?",
                ],
                task_prompt: "Enter the term for the event where the block subsidy is cut in half.",
                accepted_answers: &["halving", "the halving", "halvening"],
                hints: &[
                    "The reward is divided by two.",
                    "This event occurs every 210,000 blocks and enforces Bitcoin's disinflationary schedule.",
                ],
                success_message: "Halving — the heartbeat of Bitcoin's disinflationary schedule.",
                xp: 20,
                time_limit_secs: 20,
            },
            // ---------------------------------------------------------------
            // Chapter 31 — 21 Million
            // ---------------------------------------------------------------
            Chapter {
                title: "21 Million",
                scene_art: &[
                    r"      ┌───────────────┐          ",
                    r"      │   21,000,000  │          ",
                    r"      │    21 M       │          ",
                    r"      │  [ HARD CAP ] │          ",
                    r"      └───────────────┘          ",
                    r"         🔒 unchangeable          ",
                    r"                                 ",
                    r"    🏦 mathematical scarcity      ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "Unlike fiat currencies, Bitcoin has a hard cap coded into the protocol.",
                    "There will never be more than this many bitcoin.",
                    "What is the maximum supply?",
                ],
                task_prompt: "Enter the maximum number of bitcoin that will ever exist.",
                accepted_answers: &["21000000", "21 million", "21,000,000"],
                hints: &[
                    "The limit is hard-coded in the consensus rules and cannot be changed without a fork.",
                    "You can write it as 21000000 or 21 million.",
                ],
                success_message: "21 million. Scarce by design. No printing press, no central bank.",
                xp: 20,
                time_limit_secs: 45,
            },
            // ---------------------------------------------------------------
            // Chapter 32 — Lightning Network
            // ---------------------------------------------------------------
            Chapter {
                title: "Off-Chain Payments",
                scene_art: &[
                    r"         ⚡                       ",
                    r"        / │ \                     ",
                    r"       /  │  \                    ",
                    r"      ●───●───●                   ",
                    r"      channel → channel           ",
                    r"      instant & cheap             ",
                    r"                                 ",
                    r"    ⚡ off-chain scaling          ",
                ],
                npc_name: "Elena",
                npc_dialogue: &[
                    "The base chain is powerful but slow and expensive for tiny payments.",
                    "A second-layer network sits on top of Bitcoin, enabling instant, cheap transfers.",
                    "What's it called?",
                ],
                task_prompt: "Enter the name of Bitcoin's most prominent second-layer payment network.",
                accepted_answers: &["lightning network", "lightning", "the lightning network"],
                hints: &[
                    "Fast as... well, you know.",
                    "Payment channels enable near-instant transfers without committing every payment to the base chain.",
                ],
                success_message: "Lightning Network — scaling Bitcoin without sacrificing decentralization.",
                xp: 25,
                time_limit_secs: 20,
            },
            // ---------------------------------------------------------------
            // Chapter 33 — Not Your Keys
            // ---------------------------------------------------------------
            Chapter {
                title: "Not Your Keys",
                scene_art: &[
                    r"      ┌─────────────┐            ",
                    r"      │ EXCHANGE    │            ",
                    r"      │  (risky)    │            ",
                    r"      └─────────────┘            ",
                    r"            vs                   ",
                    r"      🗝️  in your hand           ",
                    r"      = true ownership           ",
                    r"    🔐 self-custody wins         ",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "We've covered a lot. Let me leave you with the most important rule in Bitcoin.",
                    "If someone else holds your private keys, they hold your coins.",
                    "Finish the famous phrase: 'Not your keys...'",
                ],
                task_prompt: "Complete the phrase: Not your keys, not your...",
                accepted_answers: &["coins", "not your coins"],
                hints: &[
                    "What do you call the units of currency?",
                    "The full phrase is a reminder that custody requires control of your own private keys.",
                ],
                success_message: "Not your keys, not your coins. Self-custody is self-sovereignty.",
                xp: 25,
                time_limit_secs: 45,
            },
            // ---------------------------------------------------------------
            // Chapter 34 — Mining Pools
            // ---------------------------------------------------------------
            Chapter {
                title: "Collaborative Mining",
                scene_art: &[
                    r"      ⛏️  ⛏️  ⛏️                 ",
                    r"      \  │  /                    ",
                    r"       \ │ /                     ",
                    r"      ┌──┴──┐                   ",
                    r"      │POOL │                   ",
                    r"      │share│ reward            ",
                    r"      └─────┘                   ",
                    r"    🤝 combined hash power       ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "Solo mining is a lottery with terrible odds. Most miners band together to find blocks more regularly.",
                    "When the pool finds a block, the reward is split among participants based on work contributed.",
                    "What do we call these collaborative mining groups?",
                ],
                task_prompt: "Enter the term for groups of miners who share block rewards.",
                accepted_answers: &["mining pool", "mining pools", "pool"],
                hints: &[
                    "They pool their computing resources together.",
                    "Miners combine hash power and split rewards proportionally based on shares submitted.",
                ],
                success_message: "Mining pool — strength in numbers for the proof-of-work race.",
                xp: 20,
                time_limit_secs: 20,
            },
            // ---------------------------------------------------------------
            // Chapter 35 — 51% Attack
            // ---------------------------------------------------------------
            Chapter {
                title: "51% Attack",
                scene_art: &[
                    r"      ┌─────────────┐            ",
                    r"      │  51% hash   │            ",
                    r"      │  majority   │            ",
                    r"      │  ⚠️  danger  │            ",
                    r"      │  re-org?    │            ",
                    r"      └─────────────┘            ",
                    r"      majority power threat      ",
                    r"                                 ",
                    r"    ⚠️  consensus risk           ",
                ],
                npc_name: "Dr. Hal",
                npc_dialogue: &[
                    "If a single entity controls more than half of the network's mining power, they can rewrite recent history.",
                    "This is the most famous theoretical attack on proof-of-work blockchains.",
                    "What percentage of hash rate is the critical threshold?",
                ],
                task_prompt: "Enter the percentage of hash power that enables a majority attack.",
                accepted_answers: &["51%", "51", "51 percent", "fifty one percent"],
                hints: &[
                    "More than half.",
                    "Controlling just over half the hash rate allows an attacker to outpace honest miners.",
                ],
                success_message: "51% — the theoretical tipping point where miners could outpace the honest chain.",
                xp: 25,
                time_limit_secs: 45,
            },
            // ---------------------------------------------------------------
            // Chapter 36 — Hardware Wallet
            // ---------------------------------------------------------------
            Chapter {
                title: "Air-Gapped Vault",
                scene_art: &[
                    r"      ┌─────────┐                ",
                    r"      │  USB    │                ",
                    r"      │  [🔒]   │                ",
                    r"      │  keys   │                ",
                    r"      │  never  │                ",
                    r"      │  online │                ",
                    r"      └─────────┘                ",
                    r"    🛡️  air-gapped security       ",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "The safest way to hold large amounts of bitcoin is on a dedicated device that never exposes your private keys to the internet.",
                    "What do we call this physical security device?",
                ],
                task_prompt: "Enter the term for a physical device that stores private keys offline.",
                accepted_answers: &["hardware wallet", "cold wallet", "hardware device"],
                hints: &[
                    "It's a piece of hardware dedicated to holding your wallet.",
                    "Private keys are generated and stored on a dedicated device that never connects to the internet.",
                ],
                success_message: "Hardware wallet — your private keys never touch the internet.",
                xp: 25,
                time_limit_secs: 45,
            },
        ],
    }
}
