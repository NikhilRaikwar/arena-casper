import { createArenaClient } from "../agents/shared/arena-client.js";
import { agentAccount, loadConfig } from "../agents/shared/config.js";
import { sha256 } from "../agents/shared/hash.js";

const config = loadConfig();
const client = createArenaClient(config);
const matchId = process.env.MATCH_ID ? Number(process.env.MATCH_ID) : undefined;
const match = matchId ? await client.getMatch(matchId) : await client.getLatestMatch();

if (!match) throw new Error("No match found to settle.");
if (match.status !== "active") throw new Error(`Match ${match.id} is ${match.status}, not active.`);

const settlementHash = sha256(JSON.stringify({
  matchId: match.id,
  finalValueA: match.valueA,
  finalValueB: match.valueB,
  verifier: agentAccount("verifier")
}));

const settled = await client.settleMatch({
  matchId: match.id,
  caller: agentAccount("verifier"),
  finalValueA: match.valueA,
  finalValueB: match.valueB,
  settlementHash
});

console.log(`Settled match ${settled.id}`);
console.log(`Winner: ${settled.winner ?? "draw"}`);
console.log(`Deploy: ${settled.settleDeployHash ?? "live deploy"}`);
