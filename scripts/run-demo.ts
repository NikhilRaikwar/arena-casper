import { spawn } from "node:child_process";
import { createArenaClient } from "../agents/shared/arena-client.js";
import { createAndStartDemoMatch, runAgent } from "../agents/shared/agent-runner.js";
import { agentAccount, loadConfig } from "../agents/shared/config.js";
import { sha256 } from "../agents/shared/hash.js";

console.log("🚀 Starting Arena Spectator Server...");
const spectator = spawn("npx", ["tsx", "api/server.ts"], {
  stdio: "inherit",
  shell: true
});

// Give the server 2 seconds to initialize and bind to the port
await new Promise((resolve) => setTimeout(resolve, 2000));

console.log("\n⚡ Spectator Server is running at http://localhost:3001");
console.log("--------------------------------------------------");
console.log("🏁 Initializing Match on Casper network...");

const matchId = await createAndStartDemoMatch();

console.log(`\n🤖 Running Agents (Alpha & Beta) concurrently in real-time...`);
await Promise.all([
  runAgent({ agentId: "alpha", matchId, iterations: 6 }),
  runAgent({ agentId: "beta", matchId, iterations: 6 })
]);

console.log("\n⚖️ Settling Match on-chain...");
const client = createArenaClient(loadConfig());
const match = await client.getMatch(matchId);
if (!match) throw new Error(`Match ${matchId} disappeared`);

const settlementEvidence = JSON.stringify({
  matchId,
  valueA: match.valueA,
  valueB: match.valueB,
  settledAt: new Date().toISOString()
});

const settled = await client.settleMatch({
  matchId,
  caller: agentAccount("verifier"),
  finalValueA: match.valueA,
  finalValueB: match.valueB,
  settlementHash: sha256(settlementEvidence)
});

console.log("--------------------------------------------------");
console.log(`🏆 Match #${settled.id} Settled Successfully!`);
console.log(`Winner: ${settled.winner ? settled.winner.toUpperCase() : "DRAW"}`);
console.log(`Final Alpha Value: ${Math.round(settled.valueA / 1e9).toLocaleString()} CSPR`);
console.log(`Final Beta Value: ${Math.round(settled.valueB / 1e9).toLocaleString()} CSPR`);
console.log(`Settlement Deploy Hash: ${settled.settleDeployHash ?? "mock"}`);
console.log("--------------------------------------------------");
console.log("👉 The dashboard is live. Open http://localhost:3001 in your browser.");
console.log("Press Ctrl+C to stop the Spectator Server.");

// Keep the process alive so the spectator server stays open for the user to view
await new Promise(() => {});
