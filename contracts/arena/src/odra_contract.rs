use odra::prelude::*;

#[odra::odra_type]
pub enum OdraMatchStatus {
    Pending,
    Active,
    Settled,
}

#[odra::module]
pub struct ArenaContractModule {
    next_match_id: Var<u64>,
    creator: Mapping<u64, String>,
    agent_a: Mapping<u64, String>,
    agent_b: Mapping<u64, String>,
    verifier: Mapping<u64, String>,
    start_block: Mapping<u64, u64>,
    end_block: Mapping<u64, u64>,
    start_budget: Mapping<u64, u64>,
    status: Mapping<u64, OdraMatchStatus>,
    value_a: Mapping<u64, u64>,
    value_b: Mapping<u64, u64>,
    trade_count: Mapping<u64, u32>,
    winner: Mapping<u64, Option<String>>,
    settlement_hash: Mapping<u64, Option<String>>,
}

#[odra::module]
impl ArenaContractModule {
    pub fn init(&mut self) {
        self.next_match_id.set(1);
    }

    pub fn create_match(
        &mut self,
        creator: String,
        agent_a: String,
        agent_b: String,
        verifier: String,
        current_block: u64,
        duration_blocks: u64,
        start_budget: u64,
    ) -> u64 {
        self.require(duration_blocks > 0, "duration must be positive");
        self.require(start_budget > 0, "start budget must be positive");
        self.require(agent_a != agent_b, "agent accounts must be distinct");

        let id = self.next_match_id.get_or_default();
        self.next_match_id.set(id + 1);
        self.creator.set(&id, creator);
        self.agent_a.set(&id, agent_a);
        self.agent_b.set(&id, agent_b);
        self.verifier.set(&id, verifier);
        self.start_block.set(&id, current_block);
        self.end_block.set(&id, current_block + duration_blocks);
        self.start_budget.set(&id, start_budget);
        self.status.set(&id, OdraMatchStatus::Pending);
        self.value_a.set(&id, start_budget);
        self.value_b.set(&id, start_budget);
        self.trade_count.set(&id, 0);
        self.winner.set(&id, None);
        self.settlement_hash.set(&id, None);
        id
    }

    pub fn start_match(&mut self, match_id: u64, caller: String, current_block: u64) {
        let match_creator = self.creator.get(&match_id).unwrap_or_default();
        let verifier = self.verifier.get(&match_id).unwrap_or_default();
        self.require(caller == match_creator || caller == verifier, "unauthorized start caller");
        self.require(
            self.status.get(&match_id) == Some(OdraMatchStatus::Pending),
            "match is not pending",
        );
        let old_start = self.start_block.get(&match_id).unwrap_or_default();
        let old_end = self.end_block.get(&match_id).unwrap_or_default();
        let duration = old_end.saturating_sub(old_start);
        self.start_block.set(&match_id, current_block);
        self.end_block.set(&match_id, current_block + duration);
        self.status.set(&match_id, OdraMatchStatus::Active);
    }

    pub fn record_trade(
        &mut self,
        match_id: u64,
        caller: String,
        action: String,
        pair: String,
        amount: u64,
        price: u64,
        portfolio_value: u64,
        reasoning_hash: String,
        evidence_hash: String,
        current_block: u64,
    ) {
        self.require(!action.is_empty(), "action is required");
        self.require(!pair.is_empty(), "pair is required");
        self.require(!reasoning_hash.is_empty(), "reasoning hash is required");
        self.require(!evidence_hash.is_empty(), "evidence hash is required");
        self.require(
            self.status.get(&match_id) == Some(OdraMatchStatus::Active),
            "match is not active",
        );
        let end_block = self.end_block.get(&match_id).unwrap_or_default();
        self.require(current_block <= end_block, "match already ended");

        let agent_a = self.agent_a.get(&match_id).unwrap_or_default();
        let agent_b = self.agent_b.get(&match_id).unwrap_or_default();
        self.require(caller == agent_a || caller == agent_b, "unauthorized agent");

        if caller == agent_a {
            self.value_a.set(&match_id, portfolio_value);
        } else {
            self.value_b.set(&match_id, portfolio_value);
        }
        let count = self.trade_count.get(&match_id).unwrap_or_default();
        self.trade_count.set(&match_id, count + 1);

        let evidence_digest = self.digest_trade(action, pair, amount, price, portfolio_value);
        self.settlement_hash.set(&match_id, Some(evidence_digest));
    }

    pub fn settle_match(
        &mut self,
        match_id: u64,
        caller: String,
        current_block: u64,
        final_value_a: u64,
        final_value_b: u64,
        settlement_hash: String,
    ) -> Option<String> {
        self.require(!settlement_hash.is_empty(), "settlement hash is required");
        let verifier = self.verifier.get(&match_id).unwrap_or_default();
        self.require(caller == verifier, "unauthorized verifier");
        self.require(
            self.status.get(&match_id) == Some(OdraMatchStatus::Active),
            "match is not active",
        );
        let end_block = self.end_block.get(&match_id).unwrap_or_default();
        self.require(current_block >= end_block, "too early to settle");

        self.value_a.set(&match_id, final_value_a);
        self.value_b.set(&match_id, final_value_b);
        self.status.set(&match_id, OdraMatchStatus::Settled);
        self.settlement_hash.set(&match_id, Some(settlement_hash));

        let winner = if final_value_a > final_value_b {
            self.agent_a.get(&match_id)
        } else if final_value_b > final_value_a {
            self.agent_b.get(&match_id)
        } else {
            None
        };
        self.winner.set(&match_id, winner.clone());
        winner
    }

    pub fn get_status(&self, match_id: u64) -> Option<OdraMatchStatus> {
        self.status.get(&match_id)
    }

    pub fn get_value_a(&self, match_id: u64) -> u64 {
        self.value_a.get(&match_id).unwrap_or_default()
    }

    pub fn get_value_b(&self, match_id: u64) -> u64 {
        self.value_b.get(&match_id).unwrap_or_default()
    }

    pub fn get_start_block(&self, match_id: u64) -> u64 {
        self.start_block.get(&match_id).unwrap_or_default()
    }

    pub fn get_end_block(&self, match_id: u64) -> u64 {
        self.end_block.get(&match_id).unwrap_or_default()
    }

    pub fn get_agent_a(&self, match_id: u64) -> Option<String> {
        self.agent_a.get(&match_id)
    }

    pub fn get_agent_b(&self, match_id: u64) -> Option<String> {
        self.agent_b.get(&match_id)
    }

    pub fn get_trade_count(&self, match_id: u64) -> u32 {
        self.trade_count.get(&match_id).unwrap_or_default()
    }

    pub fn get_winner(&self, match_id: u64) -> Option<String> {
        self.winner.get(&match_id).unwrap_or_default()
    }

    fn digest_trade(
        &self,
        action: String,
        pair: String,
        amount: u64,
        price: u64,
        portfolio_value: u64,
    ) -> String {
        format!("{action}:{pair}:{amount}:{price}:{portfolio_value}")
    }

    fn require(&self, condition: bool, message: &str) {
        if !condition {
            #[cfg(target_arch = "wasm32")]
            self.env().revert(OdraError::user(1));
            #[cfg(not(target_arch = "wasm32"))]
            self.env().revert(OdraError::user(1, message));
        }
    }
}

#[cfg(test)]
mod odra_tests {
    use super::ArenaContractModule;
    use super::ArenaContractModuleHostRef;
    use super::OdraMatchStatus;
    use odra::host::{Deployer, NoArgs};

    fn deploy() -> ArenaContractModuleHostRef {
        let env = odra_test::env();
        ArenaContractModule::deploy(&env, NoArgs)
    }

    #[test]
    fn creates_and_starts_match() {
        let mut arena = deploy();
        let id = arena.create_match(
            "creator".to_string(),
            "alpha".to_string(),
            "beta".to_string(),
            "verifier".to_string(),
            10,
            20,
            1_000,
        );
        arena.start_match(id, "creator".to_string(), 12);
        assert_eq!(arena.get_status(id), Some(OdraMatchStatus::Active));
        assert_eq!(arena.get_start_block(id), 12);
        assert_eq!(arena.get_end_block(id), 32);
    }

    #[test]
    fn records_trade_and_settles_winner() {
        let mut arena = deploy();
        let id = arena.create_match(
            "creator".to_string(),
            "alpha".to_string(),
            "beta".to_string(),
            "verifier".to_string(),
            1,
            2,
            1_000,
        );
        arena.start_match(id, "creator".to_string(), 2);
        arena.record_trade(
            id,
            "alpha".to_string(),
            "BUY".to_string(),
            "CSPR/USDT".to_string(),
            100,
            25_000,
            1_100,
            "reason".to_string(),
            "evidence".to_string(),
            3,
        );
        let winner = arena.settle_match(id, "verifier".to_string(), 4, 1_100, 1_000, "settle".to_string());
        assert_eq!(winner, Some("alpha".to_string()));
        assert_eq!(arena.get_trade_count(id), 1);
    }
}
