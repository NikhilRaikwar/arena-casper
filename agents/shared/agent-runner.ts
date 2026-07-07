import { createArenaClient } from "./arena-client.js";
import { agentAccount, loadConfig } from "./config.js";
import { sha256 } from "./hash.js";
import { applyTrade, initialPortfolio, portfolioValue } from "./portfolio.js";
import { PriceFeed } from "./price-feed.js";
import { decide } from "./strategy.js";
import type { AgentId, Portfolio, PricePoint } from "./types.js";

export interface RunAgentOptions {
  agentId: AgentId;
  iterations?: number;
  matchId?: number;
}

export async function runAgent(options: RunAgentOptions): Promise<void> {
  const config = loadConfig();
  const client = createArenaClient(config);
  const match = options.matchId
    ? await client.getMatch(options.matchId)
    : await client.getLatestMatch();
  if (!match) throw new Error("No match found. Run scripts/run-demo.ts or create a match first.");
  if (match.status !== "active") throw new Error(`Match ${match.id} is ${match.status}, not active.`);

  const agent = options.agentId === "alpha" ? match.agentA : match.agentB;
  const feed = new PriceFeed();
  const history: PricePoint[] = [];
  let portfolio: Portfolio = initialPortfolio(
    options.agentId === "alpha" ? match.valueA : match.valueB,
    0.023
  );
  const iterations = options.iterations ?? 4;

  for (let i = 0; i < iterations; i += 1) {
    const price = await feed.nextPrice();
    history.push(price);
    const decision = decide(options.agentId, history, portfolio);
    portfolio = applyTrade(portfolio, decision.action, decision.amount, price.price);
    const value = portfolioValue(portfolio, price.price);
    const evidence = JSON.stringify({ price, decision, iteration: i + 1 });
    const record = await client.recordTrade({
      matchId: match.id,
      agent,
      agentId: options.agentId,
      action: decision.action,
      pair: "CSPR/USDT",
      amount: decision.amount,
      price: Math.round(price.price * 1_000_000_000),
      portfolioValue: value,
      reasoning: decision.reasoning,
      reasoningHash: sha256(decision.reasoning),
      evidenceHash: sha256(evidence)
    });
    console.log(
      `[${options.agentId}] ${record.action} value=${record.portfolioValue} deploy=${record.deployHash}`
    );
    if (i < iterations - 1) {
      await sleep(config.tickMs);
    }
  }
}

export async function createAndStartDemoMatch(): Promise<number> {
  const config = loadConfig();
  const client = createArenaClient(config);
  const creator = agentAccount("creator");
  const match = await client.createMatch({
    creator,
    agentA: agentAccount("alpha"),
    agentB: agentAccount("beta"),
    verifier: agentAccount("verifier"),
    durationBlocks: config.matchDurationBlocks,
    startBudget: config.matchStartBudget
  });
  console.log(`[match] created id=${match.id} deploy=${match.createDeployHash ?? "live"}`);
  const started = await client.startMatch(match.id, creator);
  console.log(`[match] started id=${started.id} deploy=${started.startDeployHash ?? "live"}`);
  return started.id;
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
