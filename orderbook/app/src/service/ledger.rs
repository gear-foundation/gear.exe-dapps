use sails_rs::{collections::BTreeMap, prelude::*};
pub type TokenId = H160;
pub type EthAddress = H160;

#[derive(Debug, Default, Clone)]
pub struct Account {
    pub available: BTreeMap<TokenId, U256>,
    pub reserved: BTreeMap<TokenId, U256>,
}

impl Account {
    fn balance_available(&self, token: TokenId) -> U256 {
        *self.available.get(&token).unwrap_or(&U256::zero())
    }

    fn balance_reserved(&self, token: TokenId) -> U256 {
        *self.reserved.get(&token).unwrap_or(&U256::zero())
    }
}

#[derive(Debug, Default)]
pub struct Ledger {
    pub accounts: BTreeMap<EthAddress, Account>,
}

impl Ledger {
    /// Deposit tokens to the account (increases available balance)
    pub fn deposit(&mut self, account: EthAddress, token: TokenId, amount: U256) {
        let account = self.accounts.entry(account).or_default();
        let current = account.available.entry(token).or_default();
        *current = current.checked_add(amount).expect("Deposit overflow");
    }

    /// Reserve tokens for an order (moves from available to reserved)
    pub fn reserve(&mut self, account: EthAddress, token: TokenId, amount: U256) -> bool {
        let account = self.accounts.entry(account).or_default();
        let available = account.available.entry(token).or_default();
        if *available < amount {
            return false;
        }
        *available -= amount;
        let reserved = account.reserved.entry(token).or_default();
        *reserved = reserved.checked_add(amount).expect("Reserve overflow");
        true
    }

    /// Consume reserved tokens (removes them from the account)
    pub fn consume_reserved(&mut self, actor: EthAddress, token: TokenId, amount: U256) {
        let account = self.accounts.entry(actor).or_default();
        let reserved = account.reserved.entry(token).or_default();
        assert!(
            *reserved >= amount,
            "Cannot consume more than reserved balance"
        );
        *reserved -= amount;
    }

    /// Unreserve tokens (return reserved tokens back to available)
    pub fn unreserve(&mut self, actor: EthAddress, token: TokenId, amount: U256) {
        let account = self.accounts.entry(actor).or_default();
        let reserved = account.reserved.entry(token).or_default();
        assert!(
            *reserved >= amount,
            "Cannot unreserve more than reserved balance"
        );
        *reserved -= amount;
        let available = account.available.entry(token).or_default();
        *available = available.checked_add(amount).expect("Unreserve overflow");
    }
}
