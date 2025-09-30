use sails_rs::{
    U256,
    collections::{BTreeMap, HashMap, VecDeque},
    prelude::*,
};
use strum_macros::EnumString;

/// Price precision constant (1e18 = 18 decimals)
pub const PRICE_PRECISION: U256 = U256([1_000_000_000_000_000_000, 0, 0, 0]);

/// Order identifier
pub type OrderId = u64;

/// Side of an order
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Side {
    Buy,
    Sell,
}

/// Order kind (execution rules)
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum OrderKind {
    Limit,             // Stays in the book until cancelled
    Market,            // Executes immediately at the best price
    FillOrKill,        // Must be fully executed or cancelled
    ImmediateOrCancel, // Executes partially, remainder is cancelled
}

/// Order structure
#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub kind: OrderKind,
    pub price: U256, // price in quote terms, scaled by PRICE_PRECISION
    pub amount_base: U256,
    pub reserved_amount: U256,
    pub owner: H160,
}

/// Trade event
#[derive(Debug, Clone)]
pub struct Trade {
    pub maker: H160,
    pub taker: H160,
    pub price: U256,
    pub amount_base: U256,
    pub amount_quote: U256,
}

/// Queue of orders at the same price level (FIFO)
pub type OrderQueue = VecDeque<Order>;

#[derive(Debug, Default)]
pub struct OrderBook {
    pub bids: BTreeMap<U256, OrderQueue>, // buy orders (key = price, descending)
    pub asks: BTreeMap<U256, OrderQueue>, // sell orders (key = price, ascending)
    order_index: HashMap<OrderId, (Side, U256)>,
    next_order_id: OrderId,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            order_index: HashMap::new(),
            next_order_id: 1,
        }
    }

    pub fn gen_order_id(&mut self) -> OrderId {
        let id = self.next_order_id;
        self.next_order_id += 1;
        id
    }

    /// Insert a limit order into the book
    fn insert_limit_order(&mut self, order: Order) {
        let book = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        self.order_index.insert(order.id, (order.side, order.price));
        book.entry(order.price)
            .or_insert_with(VecDeque::new)
            .push_back(order);
    }

    pub fn calc_quote(base: U256, price: U256, round_up: bool) -> U256 {
        let mul = base.checked_mul(price).expect("Multiplication overflow");
        let mut result = mul / PRICE_PRECISION;

        if round_up {
            let remainder = mul % PRICE_PRECISION;
            if !remainder.is_zero() {
                result = result.checked_add(U256::one()).expect("Addition overflow");
            }
        }

        result
    }

    fn simulate_match(&self, order: &Order) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut remaining = order.amount_base;

        match order.side {
            Side::Buy => {
                for (price, queue) in &self.asks {
                    if *price > order.price {
                        break;
                    }
                    for existing in queue {
                        if remaining.is_zero() {
                            break;
                        }
                        let trade_amount = remaining.min(existing.amount_base);
                        let quote = Self::calc_quote(trade_amount, *price, false);
                        trades.push(Trade {
                            maker: existing.owner,
                            taker: order.owner,
                            price: *price,
                            amount_base: trade_amount,
                            amount_quote: quote,
                        });
                        remaining -= trade_amount;
                    }
                }
            }
            Side::Sell => {
                for (price, queue) in self.bids.iter().rev() {
                    if *price < order.price {
                        break;
                    }
                    for existing in queue {
                        if remaining.is_zero() {
                            break;
                        }
                        let trade_amount = remaining.min(existing.amount_base);
                        let quote = Self::calc_quote(trade_amount, *price, false);
                        trades.push(Trade {
                            maker: existing.owner,
                            taker: order.owner,
                            price: *price,
                            amount_base: trade_amount,
                            amount_quote: quote,
                        });
                        remaining -= trade_amount;
                    }
                }
            }
        }
        trades
    }

    /// Match an order against the book
    fn match_order(&mut self, order: &mut Order, consume_all: bool) -> Vec<Trade> {
        let mut trades = Vec::new();

        match order.side {
            Side::Buy => {
                let mut to_remove = Vec::new();
                for (price, queue) in &mut self.asks {
                    if !consume_all && *price > order.price {
                        break;
                    }
                    while let Some(mut existing) = queue.pop_front() {
                        if order.amount_base.is_zero() {
                            queue.push_front(existing);
                            break;
                        }
                        let trade_amount = order.amount_base.min(existing.amount_base);
                        let quote = Self::calc_quote(trade_amount, *price, false);
                        trades.push(Trade {
                            maker: existing.owner,
                            taker: order.owner,
                            price: *price,
                            amount_base: trade_amount,
                            amount_quote: quote,
                        });
                        order.amount_base = order
                            .amount_base
                            .checked_sub(trade_amount)
                            .expect("Order amount_base underflow");
                        order.reserved_amount = order
                            .reserved_amount
                            .checked_sub(quote)
                            .expect("Order reserved_amount underflow");
                        existing.amount_base = existing
                            .amount_base
                            .checked_sub(trade_amount)
                            .expect("Existing amount_base underflow");
                        existing.reserved_amount = existing
                            .reserved_amount
                            .checked_sub(trade_amount)
                            .expect("Existing reserved_amount underflow");

                        if !existing.amount_base.is_zero() {
                            queue.push_front(existing);
                            break;
                        }
                    }
                    if queue.is_empty() {
                        to_remove.push(*price);
                    }
                    if order.amount_base.is_zero() {
                        break;
                    }
                }
                for price in to_remove {
                    self.asks.remove(&price);
                }
            }
            Side::Sell => {
                let mut to_remove = Vec::new();
                for (price, queue) in self.bids.iter_mut().rev() {
                    if !consume_all && *price < order.price {
                        break;
                    }
                    while let Some(mut existing) = queue.pop_front() {
                        if order.amount_base.is_zero() {
                            queue.push_front(existing);
                            break;
                        }
                        let trade_amount = order.amount_base.min(existing.amount_base);
                        let quote = Self::calc_quote(trade_amount, *price, false);
                        trades.push(Trade {
                            maker: existing.owner,
                            taker: order.owner,
                            price: *price,
                            amount_base: trade_amount,
                            amount_quote: quote,
                        });
                        order.amount_base = order
                            .amount_base
                            .checked_sub(trade_amount)
                            .expect("Order amount_base underflow");
                        order.reserved_amount = order
                            .reserved_amount
                            .checked_sub(trade_amount)
                            .expect("Order reserved_amount underflow");

                        existing.amount_base = existing
                            .amount_base
                            .checked_sub(trade_amount)
                            .expect("Existing amount_base underflow");
                        existing.reserved_amount = existing
                            .reserved_amount
                            .checked_sub(quote)
                            .expect("Existing reserved_amount underflow");

                        if !existing.amount_base.is_zero() {
                            queue.push_front(existing);
                            break;
                        }
                    }
                    if queue.is_empty() {
                        to_remove.push(*price);
                    }
                    if order.amount_base.is_zero() {
                        break;
                    }
                }
                for price in to_remove {
                    self.bids.remove(&price);
                }
            }
        }

        trades
    }

    pub fn place_order(&mut self, mut order: Order) -> Vec<Trade> {
        let trades = match order.kind {
            OrderKind::Limit => {
                let executed = self.match_order(&mut order, false);
                if order.amount_base > U256::zero() {
                    self.insert_limit_order(order);
                }
                executed
            }
            OrderKind::Market => self.match_order(&mut order, true),
            OrderKind::FillOrKill => {
                let preview = self.simulate_match(&order);
                let total_base: U256 = preview
                    .iter()
                    .fold(U256::zero(), |acc, t| acc + t.amount_base);

                if total_base == order.amount_base {
                    self.match_order(&mut order, false)
                } else {
                    vec![]
                }
            }
            OrderKind::ImmediateOrCancel => {
                let executed = self.match_order(&mut order, false);
                executed
            }
        };
        trades
    }

    pub fn cancel_order(&mut self, order_id: OrderId) -> Option<Order> {
        if let Some((side, price)) = self.order_index.remove(&order_id) {
            let book = match side {
                Side::Buy => &mut self.bids,
                Side::Sell => &mut self.asks,
            };

            if let Some(orders_at_price) = book.get_mut(&price) {
                if let Some(pos) = orders_at_price.iter().position(|o| o.id == order_id) {
                    let removed_order = orders_at_price.remove(pos);

                    if orders_at_price.is_empty() {
                        book.remove(&price);
                    }

                    return removed_order;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    extern crate std;
    #[cfg(test)]
    use std::println;
    const ACTOR_1: H160 = H160([1u8; 20]);
    const ACTOR_2: H160 = H160([2u8; 20]);
    const ACTOR_3: H160 = H160([3u8; 20]);

    // USDC/BTC pair: USDC - base token, BTC - quote token
    // 1 USDC = 0.0000088 BTC (BTC ~$113,636)

    /// Price in BTC per 1 USDC (18 for accuracy)
    fn btc_price(satoshis_per_usdc: u64) -> U256 {
        // satoshis_per_usdc = amount of satoshis per 1 USDC
        // 1 BTC = 100,000,000 satoshi
        // 880 satoshi = 0.0000088 BTC per 1 USDC

        U256::from(satoshis_per_usdc) * PRICE_PRECISION / 1_000_000 // (USDC decimals)
    }

    fn usdc(amount: u64) -> U256 {
        U256::from(amount) * U256::from(1_000_000) // 1e6
    }

    fn btc(amount_str: &str) -> U256 {
        let amount_f64: f64 = amount_str.parse().unwrap();
        let satoshi_amount = (amount_f64 * 1e8) as u64; // 1e8
        U256::from(satoshi_amount)
    }

    fn setup_realistic_book() -> OrderBook {
        let mut book = OrderBook::new();

        book.place_order(Order {
            id: 0,
            side: Side::Sell,
            kind: OrderKind::Limit,
            price: btc_price(885),     // 0.00000885 BTC per 1 USDC
            amount_base: usdc(10_000), // 10,000 USDC
            owner: ACTOR_1,
            reserved_amount: usdc(10_000),
        });

        book.place_order(Order {
            id: 0,
            side: Side::Sell,
            kind: OrderKind::Limit,
            price: btc_price(895),
            amount_base: usdc(15_000),
            owner: ACTOR_1,
            reserved_amount: usdc(15_000),
        });

        book.place_order(Order {
            id: 0,
            side: Side::Sell,
            kind: OrderKind::Limit,
            price: btc_price(890),
            amount_base: usdc(25_000),
            owner: ACTOR_1,
            reserved_amount: usdc(25_000),
        });

        book.place_order(Order {
            id: 0,
            side: Side::Buy,
            kind: OrderKind::Limit,
            price: btc_price(875),
            amount_base: usdc(20_000),
            owner: ACTOR_2,
            reserved_amount: OrderBook::calc_quote(usdc(20_000), btc_price(875), true),
        });

        book.place_order(Order {
            id: 0,
            side: Side::Buy,
            kind: OrderKind::Limit,
            price: btc_price(870),
            amount_base: usdc(30_000),
            owner: ACTOR_2,
            reserved_amount: OrderBook::calc_quote(usdc(30_000), btc_price(870), true),
        });

        book
    }

    #[test]
    fn test_market_buy_usdc_for_btc() {
        let mut book = setup_realistic_book();

        let order = Order {
            id: 0,
            side: Side::Buy,
            kind: OrderKind::Market,
            price: U256::zero(), // market order (price is ignored)
            amount_base: usdc(30_000),
            owner: ACTOR_3,
            reserved_amount: OrderBook::calc_quote(usdc(30_000), btc_price(895), true),
        };

        let trades = book.place_order(order);

        assert_eq!(trades.len(), 2);

        // The first trade: 10,000 USDC per 885 satoshi
        assert_eq!(trades[0].price, btc_price(885));
        assert_eq!(trades[0].amount_base, usdc(10_000));
        assert_eq!(trades[0].maker, ACTOR_1);
        assert_eq!(trades[0].taker, ACTOR_3);

        let expected_btc_1 = btc("0.0885");
        assert_eq!(trades[0].amount_quote, expected_btc_1);

        // The second trade: 20,000 USDC per 890 satoshi
        assert_eq!(trades[1].price, btc_price(890));
        assert_eq!(trades[1].amount_base, usdc(20_000));
        let expected_btc_2 = btc("0.178");
        assert_eq!(trades[1].amount_quote, expected_btc_2);

        // Ask with price 885 should be fully consumed and removed
        assert!(!book.asks.contains_key(&btc_price(885)));

        // Ask with price 890 should be partially consumed (25K - 20K = 5K remaining)
        assert!(book.asks.contains_key(&btc_price(890)));
        let remaining_890 = &book.asks[&btc_price(890)][0];
        assert_eq!(remaining_890.amount_base, usdc(5_000));

        // Ask with price 895 should remain untouched
        assert!(book.asks.contains_key(&btc_price(895)));
        let untouched_895 = &book.asks[&btc_price(895)][0];
        assert_eq!(untouched_895.amount_base, usdc(15_000));

        // Bids should not be affected
        assert_eq!(book.bids.len(), 2, "Number of bid levels should remain 2");
        assert!(book.bids.contains_key(&btc_price(875)));
        assert!(book.bids.contains_key(&btc_price(870)));
    }

    #[test]
    fn test_market_sell_usdc_for_btc() {
        let mut book = setup_realistic_book();

        let order = Order {
            id: 0,
            side: Side::Sell,
            kind: OrderKind::Market,
            price: U256::zero(),
            amount_base: usdc(40_000),
            owner: ACTOR_3,
            reserved_amount: usdc(40_000),
        };

        let trades = book.place_order(order);

        assert_eq!(trades.len(), 2);

        // The first trade: 20,000 USDC per 875 satoshi
        assert_eq!(trades[0].price, btc_price(875));
        assert_eq!(trades[0].amount_base, usdc(20_000));
        let expected_btc_1 = btc("0.175");
        assert_eq!(trades[0].amount_quote, expected_btc_1);

        // The second trade: 20,000 USDC per 870 satoshi
        assert_eq!(trades[1].price, btc_price(870));
        assert_eq!(trades[1].amount_base, usdc(20_000));
        let expected_btc_2 = btc("0.174");
        assert_eq!(trades[1].amount_quote, expected_btc_2);

        // Bid with price 875 should be fully consumed and removed
        assert!(!book.bids.contains_key(&btc_price(875)));

        // Bid with price 870 should be partially consumed (30K - 20K = 10K remained)
        assert!(book.bids.contains_key(&btc_price(870)));
        let remaining_870 = &book.bids[&btc_price(870)][0];
        assert_eq!(remaining_870.amount_base, usdc(10_000));
        assert_eq!(remaining_870.owner, ACTOR_2);

        // Asks should remain unchanged
        assert_eq!(book.asks.len(), 3);
        assert!(book.asks.contains_key(&btc_price(885)));
        assert!(book.asks.contains_key(&btc_price(890)));
        assert!(book.asks.contains_key(&btc_price(895)));

        assert_eq!(book.asks[&btc_price(885)][0].amount_base, usdc(10_000));
        assert_eq!(book.asks[&btc_price(890)][0].amount_base, usdc(25_000));
        assert_eq!(book.asks[&btc_price(895)][0].amount_base, usdc(15_000));
    }

    #[test]
    fn test_limit_buy_at_spread() {
        let mut book = setup_realistic_book();

        // Buy USDC per 880 satoshi
        let order = Order {
            id: 0,
            side: Side::Buy,
            kind: OrderKind::Limit,
            price: btc_price(880),
            amount_base: usdc(15_000),
            owner: ACTOR_3,
            reserved_amount: OrderBook::calc_quote(usdc(15_000), btc_price(880), true),
        };

        let trades = book.place_order(order);

        assert_eq!(trades.len(), 0);
        assert!(book.bids.contains_key(&btc_price(880)));

        let added_order = &book.bids[&btc_price(880)][0];
        assert_eq!(added_order.amount_base, usdc(15_000));
    }

    #[test]
    fn test_limit_buy_crosses_spread() {
        let mut book = setup_realistic_book();

        let order = Order {
            id: 0,
            side: Side::Buy,
            kind: OrderKind::Limit,
            price: btc_price(890),
            amount_base: usdc(15_000),
            owner: ACTOR_3,
            reserved_amount: OrderBook::calc_quote(usdc(15_000), btc_price(890), true),
        };

        let trades = book.place_order(order);
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, btc_price(885));
        assert_eq!(trades[0].amount_base, usdc(10_000));

        assert_eq!(trades[1].price, btc_price(890));
        assert_eq!(trades[1].amount_base, usdc(5_000));

        // Ask with price 885 should be fully consumed and removed
        assert!(!book.asks.contains_key(&btc_price(885)));

        // Ask with price 890 should be partially consumed (25K - 5K = 20K remaining)
        assert!(book.asks.contains_key(&btc_price(890)));
        let remaining_ask_890 = &book.asks[&btc_price(890)][0];
        assert_eq!(remaining_ask_890.amount_base, usdc(20_000));

        // Ask with price 895 should remain untouched
        assert!(book.asks.contains_key(&btc_price(895)));
        assert_eq!(book.asks[&btc_price(895)][0].amount_base, usdc(15_000));

        // No remainder should be added to bids since order was fully executed

        // Bids should remain unchanged
        assert_eq!(book.bids.len(), 2);
        assert!(book.bids.contains_key(&btc_price(875)));
        assert!(book.bids.contains_key(&btc_price(870)));
    }

    #[test]
    fn test_small_amounts_precision() {
        let mut book = OrderBook::new();

        let small_usdc = usdc(1) / 100; // 0.01 USDC = 10_000 units

        book.place_order(Order {
            id: 0,
            side: Side::Sell,
            kind: OrderKind::Limit,
            price: btc_price(880),
            amount_base: small_usdc,
            owner: ACTOR_1,
            reserved_amount: small_usdc,
        });

        let buy_order = Order {
            id: 0,
            side: Side::Buy,
            kind: OrderKind::Market,
            price: U256::zero(),
            amount_base: small_usdc,
            owner: ACTOR_2,
            reserved_amount: OrderBook::calc_quote(small_usdc, btc_price(890), true),
        };

        let trades = book.place_order(buy_order);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].amount_base, small_usdc);

        // 9 satoshi
        let expected_btc = btc("0.00000008");
        assert_eq!(trades[0].amount_quote, expected_btc);
    }

    #[test]
    fn test_whale_order() {
        let mut book = setup_realistic_book();

        // 1,000,000 USDC
        let whale_amount = usdc(1_000_000);

        let order = Order {
            id: 0,
            side: Side::Buy,
            kind: OrderKind::Market,
            price: U256::zero(),
            amount_base: whale_amount,
            owner: ACTOR_3,
            reserved_amount: OrderBook::calc_quote(whale_amount, btc_price(895), true),
        };

        let trades = book.place_order(order);

        // Should consume all liquidity: 10K + 25K + 15K = 50K
        assert_eq!(trades.len(), 3);

        assert_eq!(trades[0].price, btc_price(885));
        assert_eq!(trades[0].amount_base, usdc(10_000));
        assert_eq!(trades[1].price, btc_price(890));
        assert_eq!(trades[1].amount_base, usdc(25_000));
        assert_eq!(trades[2].price, btc_price(895));
        assert_eq!(trades[2].amount_base, usdc(15_000));
    }
}
