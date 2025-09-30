#![allow(static_mut_refs)]
use sails_rs::{prelude::*, str::FromStr};

pub struct OrderbookService(());
mod orderbook;
use orderbook::*;
mod ledger;

use ledger::*;

#[derive(Debug, Default)]
struct State {
    orderbook: OrderBook,
    ledger: Ledger,
    base_token: H160,
    quote_token: H160,
}

static mut STATE: Option<State> = None;

impl OrderbookService {
    pub fn new() -> Self {
        Self(())
    }
}

impl OrderbookService {
    pub fn init(base_token: [u32; 5], quote_token: [u32; 5]) -> Self {
        unsafe {
            STATE = Some(State {
                orderbook: OrderBook::new(),
                ledger: Ledger::default(),
                base_token: H160::from(U32x5(base_token)),
                quote_token: H160::from(U32x5(quote_token)),
            })
        }
        Self(())
    }
    fn get_mut(&mut self) -> &'static mut State {
        unsafe { STATE.as_mut().expect("State is not initialized") }
    }
    fn get(&self) -> &'static State {
        unsafe { STATE.as_ref().expect("State is not initialized") }
    }
}

#[sails_rs::service]
impl OrderbookService {
    #[export]
    pub fn deposit(&mut self, account: [u32; 5], token_id: [u32; 5], amount: [u64; 4]) {
        let amount = U256(amount);
        let account = H160::from(U32x5(account));
        let token_id = H160::from(U32x5(token_id));
        self.get_mut().ledger.deposit(account, token_id, amount);
        // make eth event
    }

    #[export]
    pub fn place_order(
        &mut self,
        trader: [u32; 5],
        order_side: String,
        order_kind: String,
        price: [u64; 4],
        amount_base: [u64; 4],
        amount_quote: [u64; 4], // for Buy Market orders, indicate zero for other orders
    ) {
        let state = self.get_mut();
        let trader = H160::from(U32x5(trader));
        let side = Side::from_str(&order_side).expect("Wrong type for side");
        let kind = OrderKind::from_str(&order_kind).expect("Wrong type for side");
        let price = U256(price);
        let amount_base = U256(amount_base);
        let amount_quote = U256(amount_quote);

        let reserved_amount = match side {
            Side::Buy => {
                let cost = if kind == OrderKind::Market {
                    if amount_quote.is_zero() {
                        panic!("Cant indicate zero quote amount for Buy Market order")
                    };
                    amount_quote
                } else {
                    if price.is_zero() {
                        panic!("Price cannot be zero for this order");
                    }
                    OrderBook::calc_quote(amount_base, price, true)
                };
                if !state.ledger.reserve(trader, state.quote_token, cost) {
                    panic!("Not enough funds for Buy")
                }
                cost
            }
            Side::Sell => {
                if !state.ledger.reserve(trader, state.base_token, amount_base) {
                    panic!("Not enough funds for Sell")
                }
                amount_base
            }
        };

        let mut order = Order {
            id: state.orderbook.gen_order_id(),
            side,
            price,
            owner: trader,
            kind,
            amount_base,
            reserved_amount,
        };

        let trades = state.orderbook.place_order(order);

        let mut total_used_base = U256::zero();
        let mut total_used_quote = U256::zero();

        for trade in &trades {
            total_used_base += trade.amount_base;
            total_used_quote += trade.amount_quote;
            match side {
                Side::Buy => {
                    state
                        .ledger
                        .consume_reserved(trader, state.quote_token, trade.amount_quote);
                    state
                        .ledger
                        .deposit(trader, state.base_token, trade.amount_base);

                    state
                        .ledger
                        .consume_reserved(trade.maker, state.base_token, trade.amount_base);
                    state
                        .ledger
                        .deposit(trade.maker, state.quote_token, trade.amount_quote);
                }
                Side::Sell => {
                    state
                        .ledger
                        .consume_reserved(trader, state.base_token, trade.amount_base);
                    state
                        .ledger
                        .deposit(trader, state.quote_token, trade.amount_quote);

                    state.ledger.consume_reserved(
                        trade.maker,
                        state.quote_token,
                        trade.amount_quote,
                    );
                    state
                        .ledger
                        .deposit(trade.maker, state.base_token, trade.amount_base);
                }
            }
        }

        match kind {
            OrderKind::Market | OrderKind::ImmediateOrCancel | OrderKind::FillOrKill => {
                match side {
                    Side::Buy => {
                        let unused = reserved_amount - total_used_quote;
                        if unused > U256::zero() {
                            state.ledger.unreserve(trader, state.quote_token, unused);
                        }
                    }
                    Side::Sell => {
                        let unused = amount_base - total_used_base;
                        if unused > U256::zero() {
                            state.ledger.unreserve(trader, state.base_token, unused);
                        }
                    }
                }
            }
            _ => {}
        }

        // make event (???)
    }

    #[export]
    pub fn cancel_order(&mut self, order_id: OrderId) {
        let state = self.get_mut();
        if let Some(order) = state.orderbook.cancel_order(order_id) {}
    }
}

pub struct U32x5(pub [u32; 5]);

impl From<U32x5> for H160 {
    fn from(arr: U32x5) -> Self {
        let mut bytes = [0u8; 20];
        for (i, n) in arr.0.iter().enumerate() {
            bytes[i * 4..(i + 1) * 4].copy_from_slice(&n.to_be_bytes());
        }
        H160::from(bytes)
    }
}
