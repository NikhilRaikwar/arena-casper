import { createArenaClient } from "../agents/shared/arena-client.js";
import { agentAccount, loadConfig } from "../agents/shared/config.js";

const config = loadConfig();
if (config.mode !== "live") throw new Error("Set ARENA_MODE=live before running live scripts.");

const client = createArenaClient(config);
const matchId = process.env.MATCH_ID ? Number(process.env.MATCH_ID) : (await client.getLatestMatch())?.id;
if (!matchId) throw new Error("MATCH_ID is required or create a match first.");

const match = await client.startMatch(matchId, agentAccount("creator"));
console.log(`MATCH_ID=${match.id}`);
console.log(`START_DEPLOY=${match.startDeployHash}`);
console.log(`START_URL=${config.csprLiveBaseUrl}/${match.startDeployHash}`);
