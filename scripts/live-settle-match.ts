import { createArenaClient } from "../agents/shared/arena-client.js";
import { agentAccount, loadConfig } from "../agents/shared/config.js";
import { sha256 } from "../agents/shared/hash.js";

const config = loadConfig();
if (config.mode !== "live") throw new Error("Set ARENA_MODE=live before running live scripts.");

const client = createArenaClient(config);
const matchId = process.env.MATCH_ID ? Number(process.env.MATCH_ID) : undefined;
const match = matchId ? await client.getMatch(matchId) : await client.getLatestMatch();
if (!match) throw new Error("No match found to settle.");

const settlementHash = sha256(JSON.stringify({
  matchId: match.id,
  valueA: match.valueA,
  valueB: match.valueB,
  settledAt: new Date().toISOString()
}));

const settled = await client.settleMatch({
  matchId: match.id,
  caller: agentAccount("verifier"),
  finalValueA: match.valueA,
  finalValueB: match.valueB,
  settlementHash
});

console.log(`MATCH_ID=${settled.id}`);
console.log(`WINNER=${settled.winner ?? "draw"}`);
console.log(`SETTLE_DEPLOY=${settled.settleDeployHash}`);
console.log(`SETTLE_URL=${config.csprLiveBaseUrl}/${settled.settleDeployHash}`);
