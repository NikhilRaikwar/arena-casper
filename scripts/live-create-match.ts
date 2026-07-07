import { createArenaClient } from "../agents/shared/arena-client.js";
import { agentAccount, loadConfig } from "../agents/shared/config.js";

const config = loadConfig();
if (config.mode !== "live") throw new Error("Set ARENA_MODE=live before running live scripts.");

const client = createArenaClient(config);
const match = await client.createMatch({
  creator: agentAccount("creator"),
  agentA: agentAccount("alpha"),
  agentB: agentAccount("beta"),
  verifier: agentAccount("verifier"),
  durationBlocks: config.matchDurationBlocks,
  startBudget: config.matchStartBudget
});

console.log(`MATCH_ID=${match.id}`);
console.log(`CREATE_DEPLOY=${match.createDeployHash}`);
console.log(`CREATE_URL=${config.csprLiveBaseUrl}/${match.createDeployHash}`);
