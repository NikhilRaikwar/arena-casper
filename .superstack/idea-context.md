# Casper Buildathon Idea Context

validation:
  go_no_go: go
  confidence: 0.78
  recommended_idea: Arena
  backup_idea: LaunchGuard
  demand_signals:
    - 125 submitted projects show strong demand for agentic AI, DeFi automation, x402, and Casper-native tooling.
    - The current submission corpus is crowded in RWA oracles, payment routers, risk guards, and portfolio copilots, leaving agent performance benchmarking less crowded.
    - Casper judging criteria rewards live transaction-producing prototypes and meaningful agentic systems; Arena can demonstrate both visibly.
  risks:
    - category: technical
      description: Casper contracts cannot simply mint real CSPR or introspect all external balances; use an escrowed testnet stake ledger plus agent-posted settlement verified through CSPR.cloud.
      severity: medium
    - category: market
      description: If framed as gambling or a sportsbook, judges may discount it.
      severity: high
    - category: competition
      description: Adjacent submissions exist around DeFi agents and agent competitions, so the demo must show a real league-style benchmark, not just two scripts trading.
      severity: medium
  next_steps:
    - Build Arena as an AI agent benchmark league, not betting.
    - Use Odra match escrow, CSPR.trade MCP for strategy actions, CSPR.cloud SSE for live viewer, and x402 entry fees/agent data calls.
    - Add LaunchGuard-style safety scoring as a differentiator if time allows.