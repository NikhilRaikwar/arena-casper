#![cfg_attr(target_arch = "wasm32", no_std)]

pub mod odra_contract;

#[cfg(not(target_arch = "wasm32"))]
pub use domain::*;

#[cfg(not(target_arch = "wasm32"))]
mod domain {
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

pub type MatchId = u64;
pub type Account = String;
pub type Motes = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchStatus {
    Pending,
    Active,
    Settled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Match {
    pub id: MatchId,
    pub creator: Account,
    pub agent_a: Account,
    pub agent_b: Account,
    pub verifier: Account,
    pub start_block: u64,
    pub end_block: u64,
    pub start_budget: Motes,
    pub status: MatchStatus,
    pub value_a: Motes,
    pub value_b: Motes,
    pub trade_count: u32,
    pub winner: Option<Account>,
    pub settlement_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trade {
    pub match_id: MatchId,
    pub agent: Account,
    pub action: TradeAction,
    pub pair: String,
    pub amount: Motes,
    pub price: u64,
    pub portfolio_value: Motes,
    pub reasoning_hash: String,
    pub evidence_hash: String,
    pub block_time: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeAction {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArenaEvent {
    MatchCreated {
        match_id: MatchId,
        agent_a: Account,
        agent_b: Account,
        verifier: Account,
        start_budget: Motes,
        end_block: u64,
    },
    MatchStarted {
        match_id: MatchId,
        start_block: u64,
        end_block: u64,
    },
    TradeRecorded(Trade),
    MatchSettled {
        match_id: MatchId,
        winner: Option<Account>,
        final_value_a: Motes,
        final_value_b: Motes,
        settlement_hash: String,
    },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ArenaError {
    #[error("match not found")]
    MatchNotFound,
    #[error("caller is not authorized for this action")]
    Unauthorized,
    #[error("match is not pending")]
    NotPending,
    #[error("match is not active")]
    NotActive,
    #[error("match has not reached its end block")]
    TooEarly,
    #[error("invalid match duration")]
    InvalidDuration,
    #[error("invalid start budget")]
    InvalidBudget,
    #[error("agent accounts must be distinct")]
    DuplicateAgents,
    #[error("required hash is empty")]
    EmptyHash,
}

#[derive(Debug, Default)]
pub struct ArenaContract {
    next_match_id: MatchId,
    matches: BTreeMap<MatchId, Match>,
    trades: BTreeMap<MatchId, Vec<Trade>>,
    events: Vec<ArenaEvent>,
}

impl ArenaContract {
    pub fn new() -> Self {
        Self {
            next_match_id: 1,
            matches: BTreeMap::new(),
            trades: BTreeMap::new(),
            events: Vec::new(),
        }
    }

    pub fn create_match(
        &mut self,
        caller: Account,
        agent_a: Account,
        agent_b: Account,
        verifier: Account,
        current_block: u64,
        duration_blocks: u64,
        start_budget: Motes,
    ) -> Result<MatchId, ArenaError> {
        if duration_blocks == 0 {
            return Err(ArenaError::InvalidDuration);
        }
        if start_budget == 0 {
            return Err(ArenaError::InvalidBudget);
        }
        if agent_a == agent_b {
            return Err(ArenaError::DuplicateAgents);
        }

        let id = self.next_match_id;
        self.next_match_id += 1;
        let end_block = current_block + duration_blocks;
        let arena_match = Match {
            id,
            creator: caller,
            agent_a: agent_a.clone(),
            agent_b: agent_b.clone(),
            verifier: verifier.clone(),
            start_block: current_block,
            end_block,
            start_budget,
            status: MatchStatus::Pending,
            value_a: start_budget,
            value_b: start_budget,
            trade_count: 0,
            winner: None,
            settlement_hash: None,
        };

        self.matches.insert(id, arena_match);
        self.trades.insert(id, Vec::new());
        self.events.push(ArenaEvent::MatchCreated {
            match_id: id,
            agent_a,
            agent_b,
            verifier,
            start_budget,
            end_block,
        });
        Ok(id)
    }

    pub fn start_match(
        &mut self,
        caller: &Account,
        match_id: MatchId,
        current_block: u64,
    ) -> Result<(), ArenaError> {
        let arena_match = self
            .matches
            .get_mut(&match_id)
            .ok_or(ArenaError::MatchNotFound)?;
        if caller != &arena_match.creator && caller != &arena_match.verifier {
            return Err(ArenaError::Unauthorized);
        }
        if arena_match.status != MatchStatus::Pending {
            return Err(ArenaError::NotPending);
        }
        let duration = arena_match.end_block.saturating_sub(arena_match.start_block);
        arena_match.start_block = current_block;
        arena_match.end_block = current_block + duration;
        arena_match.status = MatchStatus::Active;
        self.events.push(ArenaEvent::MatchStarted {
            match_id,
            start_block: arena_match.start_block,
            end_block: arena_match.end_block,
        });
        Ok(())
    }

    pub fn record_trade(
        &mut self,
        caller: Account,
        trade: Trade,
        current_block: u64,
    ) -> Result<(), ArenaError> {
        if trade.reasoning_hash.is_empty() || trade.evidence_hash.is_empty() {
            return Err(ArenaError::EmptyHash);
        }
        let arena_match = self
            .matches
            .get_mut(&trade.match_id)
            .ok_or(ArenaError::MatchNotFound)?;
        if arena_match.status != MatchStatus::Active {
            return Err(ArenaError::NotActive);
        }
        if current_block > arena_match.end_block {
            return Err(ArenaError::NotActive);
        }
        if caller != arena_match.agent_a && caller != arena_match.agent_b {
            return Err(ArenaError::Unauthorized);
        }
        if trade.agent != caller {
            return Err(ArenaError::Unauthorized);
        }

        if caller == arena_match.agent_a {
            arena_match.value_a = trade.portfolio_value;
        } else {
            arena_match.value_b = trade.portfolio_value;
        }
        arena_match.trade_count += 1;
        self.trades
            .entry(trade.match_id)
            .or_default()
            .push(trade.clone());
        self.events.push(ArenaEvent::TradeRecorded(trade));
        Ok(())
    }

    pub fn settle_match(
        &mut self,
        caller: &Account,
        match_id: MatchId,
        current_block: u64,
        final_value_a: Motes,
        final_value_b: Motes,
        settlement_hash: String,
    ) -> Result<Option<Account>, ArenaError> {
        if settlement_hash.is_empty() {
            return Err(ArenaError::EmptyHash);
        }
        let arena_match = self
            .matches
            .get_mut(&match_id)
            .ok_or(ArenaError::MatchNotFound)?;
        if caller != &arena_match.verifier {
            return Err(ArenaError::Unauthorized);
        }
        if arena_match.status != MatchStatus::Active {
            return Err(ArenaError::NotActive);
        }
        if current_block < arena_match.end_block {
            return Err(ArenaError::TooEarly);
        }

        arena_match.value_a = final_value_a;
        arena_match.value_b = final_value_b;
        arena_match.status = MatchStatus::Settled;
        arena_match.settlement_hash = Some(settlement_hash.clone());
        arena_match.winner = match final_value_a.cmp(&final_value_b) {
            std::cmp::Ordering::Greater => Some(arena_match.agent_a.clone()),
            std::cmp::Ordering::Less => Some(arena_match.agent_b.clone()),
            std::cmp::Ordering::Equal => None,
        };

        self.events.push(ArenaEvent::MatchSettled {
            match_id,
            winner: arena_match.winner.clone(),
            final_value_a,
            final_value_b,
            settlement_hash,
        });
        Ok(arena_match.winner.clone())
    }

    pub fn get_match(&self, match_id: MatchId) -> Option<&Match> {
        self.matches.get(&match_id)
    }

    pub fn get_trade_count(&self, match_id: MatchId) -> u32 {
        self.matches
            .get(&match_id)
            .map(|m| m.trade_count)
            .unwrap_or_default()
    }

    pub fn get_winner(&self, match_id: MatchId) -> Option<&Account> {
        self.matches.get(&match_id).and_then(|m| m.winner.as_ref())
    }

    pub fn events(&self) -> &[ArenaEvent] {
        &self.events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account(name: &str) -> Account {
        name.to_string()
    }

    fn trade(match_id: MatchId, agent: &str, value: Motes) -> Trade {
        Trade {
            match_id,
            agent: account(agent),
            action: TradeAction::Buy,
            pair: "CSPR/USDT".to_string(),
            amount: 100,
            price: 25_000,
            portfolio_value: value,
            reasoning_hash: "reasoning".to_string(),
            evidence_hash: "evidence".to_string(),
            block_time: 2,
        }
    }

    #[test]
    fn create_match_initializes_state() {
        let mut arena = ArenaContract::new();
        let id = arena
            .create_match(
                account("creator"),
                account("alpha"),
                account("beta"),
                account("verifier"),
                10,
                20,
                1_000,
            )
            .unwrap();
        let m = arena.get_match(id).unwrap();
        assert_eq!(m.status, MatchStatus::Pending);
        assert_eq!(m.value_a, 1_000);
        assert_eq!(m.value_b, 1_000);
        assert_eq!(m.end_block, 30);
        assert_eq!(arena.events().len(), 1);
    }

    #[test]
    fn non_agent_cannot_record_trade() {
        let mut arena = ArenaContract::new();
        let id = arena
            .create_match(
                account("creator"),
                account("alpha"),
                account("beta"),
                account("verifier"),
                1,
                10,
                1_000,
            )
            .unwrap();
        arena.start_match(&account("creator"), id, 2).unwrap();
        let err = arena.record_trade(account("intruder"), trade(id, "intruder", 1_001), 3);
        assert_eq!(err, Err(ArenaError::Unauthorized));
    }

    #[test]
    fn agent_cannot_record_before_start() {
        let mut arena = ArenaContract::new();
        let id = arena
            .create_match(
                account("creator"),
                account("alpha"),
                account("beta"),
                account("verifier"),
                1,
                10,
                1_000,
            )
            .unwrap();
        let err = arena.record_trade(account("alpha"), trade(id, "alpha", 1_001), 2);
        assert_eq!(err, Err(ArenaError::NotActive));
    }

    #[test]
    fn verifier_only_settlement_and_winner_selection() {
        let mut arena = ArenaContract::new();
        let id = arena
            .create_match(
                account("creator"),
                account("alpha"),
                account("beta"),
                account("verifier"),
                1,
                2,
                1_000,
            )
            .unwrap();
        arena.start_match(&account("creator"), id, 2).unwrap();
        let unauthorized = arena.settle_match(&account("alpha"), id, 4, 1_100, 990, "settle".into());
        assert_eq!(unauthorized, Err(ArenaError::Unauthorized));
        let winner = arena
            .settle_match(&account("verifier"), id, 4, 1_100, 990, "settle".into())
            .unwrap();
        assert_eq!(winner, Some(account("alpha")));
        assert_eq!(arena.get_winner(id), Some(&account("alpha")));
    }

    #[test]
    fn tie_emits_no_winner() {
        let mut arena = ArenaContract::new();
        let id = arena
            .create_match(
                account("creator"),
                account("alpha"),
                account("beta"),
                account("verifier"),
                1,
                2,
                1_000,
            )
            .unwrap();
        arena.start_match(&account("creator"), id, 2).unwrap();
        let winner = arena
            .settle_match(&account("verifier"), id, 4, 1_000, 1_000, "settle".into())
            .unwrap();
        assert_eq!(winner, None);
    }
}
}
