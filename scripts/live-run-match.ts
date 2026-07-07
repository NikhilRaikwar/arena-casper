import { runAgent } from "../agents/shared/agent-runner.js";
import { createArenaClient } from "../agents/shared/arena-client.js";
import { loadConfig } from "../agents/shared/config.js";

const config = loadConfig();
if (config.mode !== "live") throw new Error("Set ARENA_MODE=live before running live scripts.");

const client = createArenaClient(config);
const matchId = process.env.MATCH_ID ? Number(process.env.MATCH_ID) : (await client.getLatestMatch())?.id;
if (!matchId) throw new Error("MATCH_ID is required or create/start a match first.");

const iterations = process.env.AGENT_ITERATIONS ? Number(process.env.AGENT_ITERATIONS) : 1;
await runAgent({ agentId: "alpha", matchId, iterations });
await runAgent({ agentId: "beta", matchId, iterations });
