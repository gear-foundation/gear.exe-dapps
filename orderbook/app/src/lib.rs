#![no_std]

use sails_rs::prelude::*;
pub mod service;
use service::OrderbookService;
pub struct OrderbookProgram(());

#[sails_rs::program]
impl OrderbookProgram {
    // Program's constructor
    pub fn new(base_token: [u32; 5], quote_token: [u32; 5]) -> Self {
        OrderbookService::init(base_token, quote_token);
        Self(())
    }

    // Exposed service
    pub fn orderbook(&self) -> OrderbookService {
        OrderbookService::new()
    }
}
