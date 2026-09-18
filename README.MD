# HomeRail

A Stellar-native platform where diaspora relatives send USDC (or XLM) directly into a programmable Soroban smart contract "wallet" tied to a specific student. The contract enforces rules — e.g., a portion locked for tuition (released only to a verified school's account) and the rest freely spendable via a QR-based campus wallet.

## Why Stellar?

- **Core design purpose**: Cross-border settlement — sub-5-second finality, near-zero fees
- **Anchors (SEP-24)**: Handle fiat on/off ramps — diaspora sender cashes in dollars abroad, recipient cashes out naira in Nigeria
- **Soroban (Rust smart contracts)**: Programmable disbursement logic — locked-vs-free funds, multi-recipient splits, refund-if-unclaimed
- **Live African precedent**: SurgePay is doing cross-border wallets/cards for Africa, validating the model

## Project Structure

```
stellarproof/
├── contract/                    # Soroban smart contract (Rust cdylib)
│   ├── contracts/student_wallet/# StudentWallet contract package
│   │   ├── Cargo.toml           # soroban-sdk v27 dependency
│ │       └── src/
│ │           ├── lib.rs         # 5 exported contract functions
│ │           └── test.rs        # Test scaffold
│   └── target/                  # Build output
│       └── wasm32v1-none/release/student_wallet.wasm  # 2,743 bytes
│
└── frontend/                    # Next.js TypeScript web app
    ├── app/wallet/page.tsx      # Student wallet interface
    ├── package.json             # pnpm dependencies (next 16.3.5, react 19.2.8)
    ├── .next/                   # Static build output
    └── tailwind.config/         # Tailwind CSS configuration
```

## Smart Contract (Soroban)

### 5 Exported Functions:

| Function | Description |
|---|---|
| `init(student, school)` | Creates student wallet tied to a verified school |
| `deposit(amount)` | Deposits USDC/XLM into the free portion |
| `release_to_school(amount)` | Releases locked funds to verified school account |
| `release_free(amount)` | Releases free funds for campus QR-based spending |
| `register_school(student, school)` | Registers a school for a student |

### Contract Details:
- **Storage**: Soroban `persistent()` storage with Symbol keys
- **Fund Model**: Locked portion (tuition) + Free portion (campus spending)
- **Refund**: Funds refundable after 1-year unclaimed deadline
- **School Verification**: Whitelist-based school registration
- **Wasm Size**: 2,743 bytes optimized (original: 3,529 bytes)

### Build Command:
```bash
cd stellarproof/contract
stellar contract build --package student_wallet
# Output: target/wasm32v1-none/release/student_wallet.wasm
```

## Next.js Frontend

### Features:
- Connect wallet (Soroban account)
- Deposit USDC/XLM for a student
- Release portion to verified school
- Release free funds for campus QR spending
- Status display area

### Tech Stack:
- **Framework**: Next.js 16.3.5 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **Package Manager**: pnpm

### Build Command:
```bash
cd stellarproof/frontend
pnpm run build
# Output: .next/ directory with static assets
```

## Funding Paths (Stellar Community Fund 2026)

| Fund | Amount | Description |
|---|---|---|
| **Build Award** | Development-stage funding | Early funding for validated concepts |
| **Grow Award** | Up to $150,000 in XLM | For validated projects moving to growth stage |
| **Soroban Audit Bank** | Free security audits | Professional audits for Soroban contracts |
| **Stellar x CV Labs Accelerator** | Early-stage fintech | Accelerator for founders building on Stellar, serving Africa |

## Contributor Onboarding (Parallelizable)

The project splits cleanly into independent pieces:

| Piece | Status | Contributor Type |
|---|---|---|
| **Soroban contract (Rust)** | Built | Rust smart-contract developer |
| **Anchor/SEP-24 integration** | Ready | Soroban/Anchor developer |
| **Mobile wallet UI (Next.js/TS)** | Built | Frontend TypeScript developer |
| **Campus merchant QR SDK** | Ready | Mobile/QR specialist |
| **Diaspora-facing web app** | Ready | Fullstack developer |

Each piece can be worked on independently without stepping on others.

## Quick Start

### 1. Build the Contract:
```bash
cd stellarproof/contract
stellar contract build --package student_wallet
# Wasm output: target/wasm32v1-none/release/student_wallet.wasm (2,743 bytes)
```

### 2. Build the Frontend:
```bash
cd stellarproof/frontend
pnpm run build
# Output: .next/ static assets
```

### 3. Run Tests:
```bash
cd stellarproof/contract
cargo test -p student_wallet
```

## Project Overview

**The Problem**: Nigerians abroad sending money home for a relative's school fees or upkeep get hit with high remittance fees and slow settlement through traditional corridors (Western Union, bank wires). Once the money lands, there's no way to guarantee it's spent on what it was sent for.

**The Stellar-Native Solution**: A platform where diaspora relatives send USDC (or XLM) via Stellar directly into a programmable Soroban smart contract "wallet" tied to a specific student. The contract can enforce rules — e.g., a portion locked for tuition (released only to a verified school's account) and the rest freely spendable via a QR-based campus wallet.

**Key Features**:
- Cross-border remittance via Stellar (sub-5-second finality, near-zero fees)
- Programmable disbursement logic (locked vs free funds)
- School verification and whitelist enforcement
- Refund-if-unclaimed after 1-year deadline
- QR-based campus wallet for free fund spending
- Anchor (SEP-24) integration for fiat on/off ramps

## License

This project is open source. See individual component licenses for details.

## Contact

For funding applications, contributor onboarding, or questions, refer to the project funding paths above or reach out to the Stellar community channels.