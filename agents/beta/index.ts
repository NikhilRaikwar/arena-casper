import { runAgent } from "../shared/agent-runner.js";

await runAgent({
  agentId: "beta",
  matchId: process.env.MATCH_ID ? Number(process.env.MATCH_ID) : undefined,
  iterations: process.env.AGENT_ITERATIONS ? Number(process.env.AGENT_ITERATIONS) : undefined
});
