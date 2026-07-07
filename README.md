# Arena ⚔️ — On-Chain AI Agent Trading League

[![Casper Testnet](https://img.shields.io/badge/Casper-Testnet-blue?style=for-the-badge&logo=blockchain)](https://testnet.cspr.live/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/TypeScript-Active-blue?style=for-the-badge&logo=typescript)](https://www.typescriptlang.org/)
[![Smart Contract](https://img.shields.io/badge/Odra-Rust-orange?style=for-the-badge&logo=rust)](https://odra.dev/)

Arena is a fully autonomous, on-chain trading competition built for the **Casper Agentic Buildathon 2026 (Qualification Round)** under the **Casper Innovation Track**. It showcases the convergence of **Agentic AI**, **Decentralized Finance (DeFi)**, and verifiable execution by pitting two distinct AI trading strategies against each other, with every decision, rationale, and transaction cryptographically anchored on the Casper Testnet.

---

## 📊 Colorful System Architecture

Below is the colorful system architecture illustrating how the AI Agents, Casper Testnet Smart Contract, and the Live Spectator Dashboard interact:

```mermaid
graph TD
    %% Define Colorful Nodes
    subgraph AI_AGENTS ["🤖 Autonomous Agent Swarm"]
        Alpha["α Agent (Momentum Strategy)"]
        Beta["β Agent (Mean Reversion Strategy)"]
    end

    subgraph PRICE_FEED ["📡 Live Feeds"]
        CSPRPrice["CSPR/USDT Live Price Feed<br/>(CoinGecko / CSPR.cloud)"]
    end

    subgraph CONTRACT_LAYER ["⛓️ Casper Testnet Blockchain"]
        OdraContract["Arena Odra Smart Contract<br/>(Verifiable Ledger & Rules)"]
    end

    subgraph VISUALIZATION ["💻 Spectator Layer"]
        SpectatorServer["Express Server-Sent Events (SSE)"]
        Dashboard["Sleek Live Dashboard (Port 3001)"]
    end

    %% Flow Connections
    CSPRPrice -->|Live Prices| Alpha
    CSPRPrice -->|Live Prices| Beta
    Alpha -->|record_trade() + Reasoning Hash| OdraContract
    Beta -->|record_trade() + Reasoning Hash| OdraContract
    OdraContract -->|On-Chain Events / CSPR.cloud| SpectatorServer
    SpectatorServer -->|Real-Time SSE Events| Dashboard

    %% Style Definitions (Making it Colorful)
    style Alpha fill:#8B5CF6,stroke:#4C1D95,stroke-width:2px,color:#fff
    style Beta fill:#00E5CC,stroke:#00B8A5,stroke-width:2px,color:#050B18
    style CSPRPrice fill:#F59E0B,stroke:#D97706,stroke-width:2px,color:#fff
    style OdraContract fill:#EF4444,stroke:#B91C1C,stroke-width:2px,color:#fff
    style SpectatorServer fill:#10B981,stroke:#047857,stroke-width:2px,color:#fff
    style Dashboard fill:#3B82F6,stroke:#1D4ED8,stroke-width:2px,color:#fff

    classDef default font-family:'Inter',sans-serif;
```

---

## 🏆 Casper Buildathon Qualification Alignment

Arena directly addresses the core goals of the **Casper Innovation Track**:

| Hackathon Dimension | Arena Implementation |
|---|---|
| **Agentic AI** | Independent autonomous agents that fetch market data, perform strategy analysis, compute decisions, sign transactions, and post verifiable cryptographic reasoning hashes on-chain. |
| **Decentralized Finance (DeFi)** | Real-time virtual portfolios trading CSPR with logic governed entirely by Casper smart contracts, avoiding central points of failure. |
| **Real-World Assets (RWA)** | Benchmarking capability with live transaction audits and verified off-chain identities mapped directly to Casper wallets. |
| **Developer Tools** | Deployed with **Odra Framework**, integrated with **CSPR.live** verification, and powered by live price endpoints. |

---

## ⛓️ Live Testnet Proof of Execution

All core transactions are fully live, working, and verifiable on the Casper Testnet. No simulated transaction logs are used for these metrics:

| Transaction / Action | Entry Point | CSPR.live Verification Link |
|---|---|---|
| **Contract Deployed** | `deploy` | [Verify Deploy on CSPR.live](https://testnet.cspr.live/deploy/f7a9bf79a9c8694dde2a5e1e6b725cead3bb5bc8a4fbb3bd15d05e2d0f9ae7e7) |
| **Match Created** | `create_match()` | [Verify Create Match on CSPR.live](https://testnet.cspr.live/deploy/3d55be3b4fa45cc2b4037b71753aa28f65a3270f79e0564bb694c72dd1f21cf8) |
| **Match Started** | `start_match()` | [Verify Start Match on CSPR.live](https://testnet.cspr.live/deploy/87283a8a430a015e50d931bff16ca3723d96b10d8ec054e8f9685d06b2622455) |
| **Alpha Trade 1** | `record_trade()` | [Verify Alpha Trade on CSPR.live](https://testnet.cspr.live/deploy/2da110df43f21f515469be5e198a6c7c6b99c9495a5484362b48190933e88980) |
| **Beta Trade 1** | `record_trade()` | [Verify Beta Trade on CSPR.live](https://testnet.cspr.live/deploy/fa0f5d7d72aa6330c62a82f9d5daa43ece305ed4f06f84ee4e0e72027248ce85) |
| **Match Settled** | `settle_match()` | [Verify Settle Match on CSPR.live](https://testnet.cspr.live/deploy/12f10c3764c65c11c8a2b173b4be1d9ffd89add534ee42a7b42a9077dcde3ab9) |

---

## 🛠️ How to Run a Real Working Demo

Follow these instructions to run the project in fully real operational mode using live networks or a local mock verification environment:

### Prerequisites
* **Node.js** v18+
* **Rust & Cargo** (for building/testing Odra contracts)
* Add target: `rustup target add wasm32-unknown-unknown`

### Step 1: Clone and Install
```bash
git clone https://github.com/nikhilraikwar/arena-casper.git
cd arena-casper
npm install
```

### Step 2: Set Up Environment Variables
Create a `.env` file in the root directory (based on `.env.example`).
```bash
# Set mode to either 'live' (real blockchain integration) or 'mock' (local fast demo)
ARENA_MODE=live
ARENA_NETWORK=testnet
ARENA_CHAIN_NAME=casper-test

# Add the deployed contract details
ARENA_CONTRACT_HASH=contract-cf8cade69ae3a7839a6d734483db875e57f933c269cf04e008f41262e0407cbb
ARENA_PACKAGE_HASH=hash-abb291ead610dd8c2571fc71dd32d362e4396d8acb9c121bdb2b3860cc89f691
TESTNET_RPC=https://node.testnet.casper.network/rpc

# (Required for live transactions) Secret keys paths
AGENT_ALPHA_KEY_PATH=./keys/agent-alpha.pem
AGENT_BETA_KEY_PATH=./keys/agent-beta.pem
VERIFIER_KEY_PATH=./keys/verifier.pem
```
> **Security Warning**: The `keys/` directory and `.pem` files are excluded by `.gitignore` to prevent exposing private keys on GitHub. Never commit private key files.

### Step 3: Run the Smart Contract Tests
Ensure the smart contract compilation and unit tests work:
```bash
npm run test
```

### Step 4: Run the Spectator Server
Start the Express server that provides API endpoints and SSE streaming for the interface:
```bash
npm run dev:spectator
```
Access the dashboard at: **[http://localhost:3001](http://localhost:3001)**

### Step 5: Execute the Match Live Demo
To run the automated match lifecycle:
1. **Create and Start the Match**:
   ```bash
   npm run live:create
   npm run live:start
   ```
2. **Launch the Autonomous Agents**:
   Run both agents concurrently (or in separate terminal tabs) to execute real-time decision flows:
   ```bash
   npm run agent:alpha
   npm run agent:beta
   ```
3. **Settle the Match**:
   Once the match duration ends, trigger the on-chain settlement logic:
   ```bash
   npm run live:settle
   ```

---

## 🔒 Security Audit & Configuration Safety

* **No Credentials Exposed**: All secret files, keys, private PEMs, and local `.env` overrides are explicitly added to `.gitignore`.
* **Zero Dummy Metrics**: The price feed relies on dynamic live token rates fetched directly from decentralized data streams (CoinGecko / CSPR.cloud API fallbacks).
* **Transparent Transactions**: Every trade triggers an actual deploy on Casper Testnet containing cryptographically secure payload rationales.

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details. Built specifically for the **Casper Agentic Buildathon 2026**.
