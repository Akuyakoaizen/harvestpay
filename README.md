# 🌾 HarvestPay

> A Soroban escrow smart contract on Stellar that releases USDC payments to farmers instantly when a cooperative confirms harvest delivery on-chain.

---

## 📋 Project Overview

**PROJECT NAME:** HarvestPay

**PROBLEM:** Small rice and corn farmers in the Philippines wait 30–90 days for cooperative payments after harvest delivery, forcing them into predatory loan cycles.

**SOLUTION:** A Soroban escrow dApp that releases USDC payments to farmers automatically when a cooperative officer confirms delivery on-chain — no banks, no waiting, no middlemen.

---

## 🌟 Stellar Features Used

- ✅ **Soroban Smart Contract** — core escrow and payment release logic
- ✅ **USDC Transfer** — stablecoin payment to farmer wallet
- ✅ **Trustline** — farmer wallet accepts USDC
- ✅ **Clawback/Compliance** — cooperative authorization enforcement

---

## 🎯 Target Users

Smallholder farmers in Luzon (Philippines) and the Mekong Delta (Vietnam) — people with basic smartphones but no bank accounts, who depend on cooperatives for crop payments.

---

## ⚙️ Core MVP Feature

Farmer submits harvest weight on-chain → Cooperative officer signs confirmation → Soroban contract releases USDC instantly to farmer's wallet.

---

## 📁 Repository Structure

```
harvestpay/
├── contracts/
│   └── hello-world/
│       ├── src/
│       │   ├── lib.rs        ← Smart contract logic
│       │   └── test.rs       ← Unit tests (3 tests)
│       ├── Cargo.toml        ← Contract dependencies
│       └── Makefile
├── Cargo.toml                ← Workspace config
├── Cargo.lock
└── README.md
```

---

## 🔧 Prerequisites

Before building or deploying, make sure you have:

- **Rust** toolchain (v1.70+)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **WASM target** for Rust
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

- **Stellar CLI** (v25.0+)
  ```bash
  cargo install --locked stellar-cli
  ```

- **Freighter Wallet** browser extension — set to Testnet
  - Download: https://freighter.app

---

## 🏗️ How to Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

Confirm the compiled WASM file exists:

```bash
ls target/wasm32-unknown-unknown/release/*.wasm
```

---

## 🧪 How to Test

```bash
cargo test
```

Expected output:
```
running 3 tests
test test::test_initialize ... ok
test test::test_submit_harvest ... ok
test test::test_confirm_and_pay ... ok
test result: ok. 3 passed; 0 failed
```

### Test Descriptions

| Test | Type | Description |
|------|------|-------------|
| `test_initialize` | Happy path | Cooperative address is correctly stored on-chain |
| `test_submit_harvest` | State verification | Farmer data (weight, amount, status) is correctly recorded |
| `test_confirm_and_pay` | Edge case | Only the registered cooperative can confirm; status updates to Paid |

---

## 🚀 How to Deploy to Testnet

**Step 1 — Create an identity:**
```bash
stellar keys generate --global my-key --network testnet
stellar keys address my-key
```

**Step 2 — Fund your testnet account:**
```bash
stellar keys fund my-key --network testnet
```

**Step 3 — Deploy the contract:**
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source my-key \
  --network testnet
```

Copy the **Contract ID** from the output (starts with `C...`).

**Step 4 — Verify on Stellar Expert:**
```
https://stellar.expert/explorer/testnet/contract/<YOUR_CONTRACT_ID>
```

---

## 💻 Sample CLI Invocations

**Initialize the contract (register cooperative):**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-key \
  --network testnet \
  -- initialize \
  --cooperative GCDKOBK5STRM6RL334VULNCDMYRQI6YQFJA4U7A42MM3HLFXMQOYQVBJ
```

**Submit a harvest (as farmer):**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-key \
  --network testnet \
  -- submit_harvest \
  --farmer GCDKOBK5STRM6RL334VULNCDMYRQI6YQFJA4U7A42MM3HLFXMQOYQVBJ \
  --weight_kg 500 \
  --amount_usdc 1000
```

**Confirm delivery and release payment (as cooperative):**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-key \
  --network testnet \
  -- confirm_and_pay \
  --cooperative GCDKOBK5STRM6RL334VULNCDMYRQI6YQFJA4U7A42MM3HLFXMQOYQVBJ
```

**View current harvest record:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-key \
  --network testnet \
  -- get_harvest
```

---

## 🌐 Deployed Contract

| Field | Value |
|-------|-------|
| **Network** | Stellar Testnet |
| **Contract ID** | `CAIGDOMMMVKQIF2TU2XYEKIPLZVMHAPWQYM3GCSNYHK4TJCJROQ6RXRO` |
| **Stellar Expert** | [View Contract](https://stellar.expert/explorer/testnet/contract/CAIGDOMMMVKQIF2TU2XYEKIPLZVMHAPWQYM3GCSNYHK4TJCJROQ6RXRO) |
| **Frontend** | Next.js + Tailwind + Freighter Wallet |

---

## 📅 Suggested MVP Timeline

| Phase | Duration | Description |
|-------|----------|-------------|
| Setup & Environment | 1 day | Install Rust, Stellar CLI, Freighter |
| Smart Contract | 2 days | Write and test Soroban contract |
| Deploy to Testnet | 1 day | Deploy and verify on Stellar Expert |
| Frontend | 2 days | Next.js UI with Freighter integration |
| Testing & Polish | 1 day | End-to-end testing and README |

---

## 🇵🇭 About

Built during the **Stellar Philippines UniTour — University of East Caloocan** bootcamp, powered by [Rise In](https://risein.com).

This project demonstrates how Soroban smart contracts on Stellar can solve real-world payment problems for underserved agricultural communities in Southeast Asia.

---

## 📄 License

MIT License

Copyright (c) 2026 HarvestPay

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.