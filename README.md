# 🪙 Escrowk — Solana Anchor Escrow Program

A decentralized escrow smart contract built with **Anchor** on Solana.

This project demonstrates a full escrow workflow using:

- Program Derived Addresses (PDAs)
- SPL Token Interface
- CPI token transfers
- Vault authority patterns

---

## ✨ Features

✅ Create escrow and deposit tokens  
✅ Accept escrow trades between maker & taker  
✅ Refund escrow if trade is cancelled  
✅ PDA-owned vault account  
✅ Token-2022 compatible (`InterfaceAccount`)  

---

## 🧠 How It Works

### 👤 Maker
Creates an escrow and deposits `mint A` tokens into a vault.

### 🤝 Taker
Accepts the escrow by sending `mint B` tokens and receives vault tokens.

### 🔁 Refund
Maker can cancel and reclaim tokens if escrow is unused.

---

## 🔄 Escrow Flow

### 1️⃣ Make


maker → vault (mint A)


- Create escrow PDA
- Create vault ATA owned by PDA
- Transfer tokens to vault

---

### 2️⃣ Take


taker → maker (mint B)
vault → taker (mint A)


- Taker sends mint B
- Vault releases mint A
- Vault closes
- Escrow closes

---

### 3️⃣ Refund


vault → maker (mint A)


- Vault transfers tokens back
- Vault closes
- Escrow closes

---

## 🧱 PDA Design

Escrow PDA:


[b"escrow", maker_pubkey, seed_u64]


Vault ATA:


ATA(mint_a, escrow_pda)


---

## 📁 Project Structure


programs/escrowk/
├── instructions/
│ ├── make.rs
│ ├── take.rs
│ └── refund.rs
├── state/
│ └── escrow.rs
└── lib.rs

tests/
└── escrowk.ts


---

## ⚙️ Requirements

- Rust
- Solana CLI
- Anchor CLI
- Node.js
- @coral-xyz/anchor

---

## 🚀 Setup

Clone repo:

```bash
git clone <https://github.com/sahkunal/anchor_escrowk>
cd escrowk

Install dependencies:

npm install

Build program:

anchor build

Run tests:

anchor test
🧪 Tests

Located in:

tests/escrowk.ts

Test scenarios include:

Mint creation

Token minting

Make escrow

Refund escrow

Take escrow

Vault + escrow account closure checks

Token balance verification

🛠 Tech Stack

Rust

Anchor Framework

Solana Web3.js

SPL Token Interface

TypeScript

⚠️ Developer Notes

Uses InterfaceAccount for Token-2022 compatibility.

.accountsStrict() requires exact account matching with Rust structs.

After adding instructions, regenerate IDL:

anchor clean
anchor build
📚 Learning Concepts

This project covers:

PDA authority

CPI token transfers

Vault escrow design

Anchor account constraints

Solana program architecture
