import type { AgentId, Portfolio, PricePoint, StrategyDecision } from "./types.js";

export function decide(agentId: AgentId, history: PricePoint[], portfolio: Portfolio): StrategyDecision {
  if (history.length < 3) {
    return {
      action: "HOLD",
      amount: 0,
      reasoning: `${agentId} is waiting for enough price history before risking capital.`
    };
  }

  const prices = history.map((p) => p.price);
  const latest = prices.at(-1)!;
  const short = average(prices.slice(-3));
  const long = average(prices.slice(-6));
  const recentHigh = Math.max(...prices.slice(-6));
  const deployableCash = Math.max(0, portfolio.cash * 0.25);
  const sellValue = Math.max(0, portfolio.asset * latest * 0.35);

  if (agentId === "alpha") {
    if (short > long * 1.002 && deployableCash > 0) {
      return {
        action: "BUY",
        amount: Math.round(deployableCash),
        reasoning: `Alpha sees positive momentum: short average ${short.toFixed(5)} is above long average ${long.toFixed(5)}.`
      };
    }
    if (short < long * 0.998 && sellValue > 0) {
      return {
        action: "SELL",
        amount: Math.round(sellValue),
        reasoning: `Alpha exits risk because short average ${short.toFixed(5)} fell below long average ${long.toFixed(5)}.`
      };
    }
    return {
      action: "HOLD",
      amount: 0,
      reasoning: `Alpha holds because momentum is not strong enough.`
    };
  }

  const dropFromHigh = (recentHigh - latest) / recentHigh;
  if (dropFromHigh > 0.015 && deployableCash > 0) {
    return {
      action: "BUY",
      amount: Math.round(deployableCash),
      reasoning: `Beta buys the dip: price is ${(dropFromHigh * 100).toFixed(2)}% below recent high.`
    };
  }
  if (dropFromHigh < 0.004 && sellValue > 0) {
    return {
      action: "SELL",
      amount: Math.round(sellValue),
      reasoning: `Beta takes profit because price recovered near the recent high.`
    };
  }
  return {
    action: "HOLD",
    amount: 0,
    reasoning: `Beta holds because mean-reversion threshold has not triggered.`
  };
}

function average(values: number[]): number {
  return values.reduce((sum, value) => sum + value, 0) / values.length;
}
