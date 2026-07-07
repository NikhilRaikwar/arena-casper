export type AgentId = "alpha" | "beta";
export type TradeAction = "BUY" | "SELL" | "HOLD";
export type MatchStatus = "pending" | "active" | "settled";

export interface ArenaConfig {
  mode: "mock" | "live";
  network: string;
  chainName: string;
  packageHash?: string;
  contractHash?: string;
  rpcUrl: string;
  csprLiveBaseUrl: string;
  creatorKeyPath?: string;
  alphaKeyPath?: string;
  betaKeyPath?: string;
  verifierKeyPath?: string;
  matchDurationBlocks: number;
  matchStartBudget: number;
  matchPollMs: number;
  tickMs: number;
}

export interface MatchState {
  id: number;
  creator: string;
  agentA: string;
  agentB: string;
  verifier: string;
  startBlock: number;
  endBlock: number;
  startBudget: number;
  status: MatchStatus;
  valueA: number;
  valueB: number;
  tradeCount: number;
  winner?: string;
  settlementHash?: string;
  createDeployHash?: string;
  startDeployHash?: string;
  settleDeployHash?: string;
}

export interface TradeRecord {
  matchId: number;
  agent: string;
  agentId: AgentId;
  action: TradeAction;
  pair: string;
  amount: number;
  price: number;
  portfolioValue: number;
  reasoning: string;
  reasoningHash: string;
  evidenceHash: string;
  blockTime: number;
  deployHash: string;
}

export interface ArenaEvent {
  id: string;
  type: "match_created" | "match_started" | "trade_recorded" | "match_settled";
  timestamp: string;
  deployHash: string;
  matchId: number;
  payload: Record<string, unknown>;
}

export interface Portfolio {
  cash: number;
  asset: number;
  lastPrice: number;
}

export interface StrategyDecision {
  action: TradeAction;
  amount: number;
  reasoning: string;
}

export interface PricePoint {
  price: number;
  source: string;
  timestamp: string;
}
