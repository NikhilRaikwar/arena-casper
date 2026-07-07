import type { Portfolio, TradeAction } from "./types.js";

export function initialPortfolio(startBudget: number, price: number): Portfolio {
  return { cash: startBudget, asset: 0, lastPrice: price };
}

export function portfolioValue(portfolio: Portfolio, price: number): number {
  return Math.round(portfolio.cash + portfolio.asset * price);
}

export function applyTrade(portfolio: Portfolio, action: TradeAction, amount: number, price: number): Portfolio {
  if (action === "BUY") {
    const spend = Math.min(amount, portfolio.cash);
    return {
      cash: portfolio.cash - spend,
      asset: portfolio.asset + spend / price,
      lastPrice: price
    };
  }
  if (action === "SELL") {
    const units = Math.min(amount / price, portfolio.asset);
    return {
      cash: portfolio.cash + units * price,
      asset: portfolio.asset - units,
      lastPrice: price
    };
  }
  return { ...portfolio, lastPrice: price };
}
