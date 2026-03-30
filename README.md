# Submit on Rise In Requirements

> Submit the following on your Rise In program page:

| Field | What to Submit |
|-------|----------------|
| **GitHub Repository** | https://github.com/Akuyakoaizen/harvestpay |
| **Contract ID** | GCDKOBK5STRM6RL334VULNCDMYRQI6YQFJA4U7A42MM3HLFXMQOYQVBJ |
| **Stellar Expert Link** | `https://stellar.expert/explorer/testnet/account/GCDKOBK5STRM6RL334VULNCDMYRQI6YQFJA4U7A42MM3HLFXMQOYQVBJ` |
| **Short Description** | A Soroban smart contract on Stellar that automates USDC payouts to farmers when a cooperative confirms harvest delivery on-chain. |

---

# Stellar Expert ScreenShot

<img width="1366" height="768" alt="image" src="https://github.com/user-attachments/assets/017981ab-01a6-4859-9642-e386ab55e0d3" />

---

## The Problem

Small-scale farmers in Southeast Asia often experience delayed payments and high remittance fees when selling harvest to cooperatives or marketplaces. Manual verification and slow payment processing cause cash flow issues for farmers and inefficiencies for cooperatives.

---

## The Solution

**HarvestPay** automates harvest submissions, confirmations, and payouts on Stellar Soroban. Farmers submit harvest details on-chain. When a cooperative confirms delivery, the contract instantly releases the agreed USDC payment to the farmer’s wallet, ensuring transparency, speed, and trustless disbursement.

---

## Stellar Features Used

| Feature | Role in HarvestPay |
|---------|-------------------|
| Soroban smart contracts | Manage harvest submissions, confirmations, and automated payouts |
| USDC (custom token) | Transfers payment to farmers securely |
| Trustlines | Farmer wallets opt-in to receive USDC |
| Contract storage | Stores harvest records and cooperative info |

---

## Target Users

| Segment | Location | Pain / Incentive |
|---------|---------|-----------------|
| Small-scale farmers | SEA (Philippines, Indonesia, Vietnam) | Receive instant, verifiable payments for delivered harvest |
| Cooperatives | SEA | Automate verification and reduce administrative overhead |
| Agri-platforms / marketplaces | Global | Ensure transparent and traceable farmer payouts |

---

## MVP Core Feature — Transaction Flow

```
Cooperative calls initialize(cooperative_address)
→ Sets cooperative in contract storage
Farmer calls submit_harvest(farmer_address, weight_kg, amount_usdc)
→ HarvestRecord stored on-chain with status Pending
→ HARVEST event emitted
Cooperative calls confirm_and_pay(cooperative_address)
→ Contract verifies cooperative auth
→ HarvestRecord status updated to Paid
→ USDC transferred to farmer wallet
→ PAYMENT event emitted
Anyone calls get_harvest()
→ Returns current HarvestRecord
Anyone calls get_cooperative()
→ Returns cooperative address
```


Demo-able end-to-end in under 2 minutes.

---

## Why This Wins

HarvestPay directly addresses slow and opaque agricultural payments. It demonstrates live USDC transfers, cooperative-controlled confirmation, automatic status updates, and an on-chain audit trail — all in one demo. Judges see an instantly functional Soroban contract with real-world impact.

---

## Optional Edge (Bonus)

- Multi-harvest support per farmer  
- Automated off-chain notifications when harvest is confirmed  
- Analytics dashboard for cooperative tracking  

---

## Suggested MVP Timeline

| Day | Milestone |
|-----|-----------|
| 1   | Write lib.rs: HarvestRecord, HarvestStatus, submit_harvest, confirm_and_pay |
| 2   | Add initialize, get_harvest, get_cooperative; write unit tests |
| 3   | Build and test contract locally |
| 4   | Deploy to Stellar testnet |
| 5   | Write CLI scripts for farmer and cooperative interactions |
| 6   | Optional: Next.js frontend with farmer dashboard and cooperative panel |
| 7   | Record demo, polish README, submit |

---

## Prerequisites

| Tool | Version |
|------|---------|
| Rust toolchain | stable >= 1.85 |
| wasm32-unknown-unknown target | rustup target add wasm32-unknown-unknown |
| Soroban CLI | cargo install --locked soroban-cli |
| Node.js (frontend) | >= 18 LTS |

---

## Build

```bash
export RUSTFLAGS="-C target-feature=-reference-types"
cargo build --target wasm32-unknown-unknown --release
```

---

## Test

```bash
cargo test
```

Expected output:

```
test test::test_initialize ... ok
test test::test_submit_harvest ... ok
test test::test_confirm_and_pay ... ok


test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

---

## Deploy to Testnet

```bash
# 1. Configure testnet
soroban config network testnet https://rpc.testnet.stellar.org

# 2. Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/harvestpay.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet

# 3. Initialize cooperative
soroban contract invoke \
  --id $CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  --network testnet \
  -- initialize \
  --cooperative $COOPERATIVE_ADDRESS
```

---

## Sample CLI Invocations

### Enroll a Student

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --source admin_keypair \
  --network testnet \
  -- enroll_student \
  --admin           GADMIN...ADDRESS \
  --student         GSTUDENT...ADDRESS \
  --milestone_count 3
```

### Sponsor Deposits Funds

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --source sponsor_keypair \
  --network testnet \
  -- deposit_funds \
  --sponsor  GSPONSOR...ADDRESS \
  --token    $XLM_TOKEN_ADDRESS \
  --student  GSTUDENT...ADDRESS \
  --amount   3000000000
```

### submit harvest

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source FARMER_KEY \
  --network testnet \
  -- submit_harvest \
  --farmer  GFARMER...ADDRESS \
  --weight_kg 500 \
  --amount_usdc 1000
```

### confirm and pay

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source COOP_KEY \
  --network testnet \
  -- confirm_and_pay \
  --cooperative GCOOP...ADDRESS
```

### Send a Cross-Border Micropayment

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --source sponsor_keypair \
  --network testnet \
  -- send_remittance \
  --sender    GSPONSOR...ADDRESS \
  --token     $XLM_TOKEN_ADDRESS \
  --receiver  GSTUDENT...ADDRESS \
  --amount    50000000 \
  --remit_id  "INV-2025-SUB-001"
```

### get harvest record

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source ANY_KEY \
  --network testnet \
  -- get_harvest
```
### get cooperative address

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source ANY_KEY \
  --network testnet \
  -- get_cooperative
```

---

## Reference Repos

Stellar Bootcamp 2026: https://github.com/armlynobinguar/Stellar-Bootcamp-2026
HarvestPay full example: https://github.com/Akuyakoaizen/harvestpay

---

## License

MIT (c) 2026 HarvestPay Contributors
