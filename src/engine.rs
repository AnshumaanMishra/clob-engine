use crate::types::{Order, OrderId, Price, Quantity, Side};
use rustc_hash::FxHashMap;
use std::collections::{BTreeMap, VecDeque};

pub struct OrderBook {
  orders: FxHashMap<OrderId, Order>,
  bids: BTreeMap<Price, VecDeque<OrderId>>,
  asks: BTreeMap<Price, VecDeque<OrderId>>,
}

impl OrderBook {
  pub fn new() -> Self {
    Self {
      orders: FxHashMap::default(),
      bids: BTreeMap::new(),
      asks: BTreeMap::new(),
    }
  }

  pub fn add_limit_order(&mut self, order: Order) {
    let remaining_qty = self.match_order(&order);

    if remaining_qty > 0 {
      let mut resting_order = order.clone();
      resting_order.quantity = remaining_qty;

      self.orders.insert(resting_order.id, resting_order.clone());

      let queue = match resting_order.side {
        Side::Buy => self
          .bids
          .entry(resting_order.price)
          .or_insert_with(VecDeque::new),
        Side::Sell => self
          .asks
          .entry(resting_order.price)
          .or_insert_with(VecDeque::new),
      };
      queue.push_back(resting_order.id);
    }
  }

  pub fn cancel_order(&mut self, order_id: OrderId) -> bool {
    self.orders.remove(&order_id).is_some()
  }

  fn match_order(&mut self, aggressive_order: &Order) -> Quantity {
    let mut remaining_qty = aggressive_order.quantity;

    match aggressive_order.side {
      Side::Buy => {
        let mut prices_to_remove = Vec::new();

        for (&ask_price, queue) in self.asks.iter_mut() {
          if ask_price > aggressive_order.price || remaining_qty == 0 {
            break;
          }

          while let Some(&resting_id) = queue.front() {
            if remaining_qty == 0 {
              break;
            }

            if let Some(resting_order) = self.orders.get_mut(&resting_id) {
              let fill_qty = std::cmp::min(remaining_qty, resting_order.quantity);
              remaining_qty -= fill_qty;
              resting_order.quantity -= fill_qty;

              if resting_order.quantity == 0 {
                queue.pop_front();
                self.orders.remove(&resting_id);
              }
            } else {
              queue.pop_front();
            }
          }
          if queue.is_empty() {
            prices_to_remove.push(ask_price);
          }
        }
        for price in prices_to_remove {
          self.asks.remove(&price);
        }
      }
      Side::Sell => {
        let mut prices_to_remove = Vec::new();

        // Iterate rev() to get the highest Bids first
        for (&bid_price, queue) in self.bids.iter_mut().rev() {
          if bid_price < aggressive_order.price || remaining_qty == 0 {
            break;
          }

          while let Some(&resting_id) = queue.front() {
            if remaining_qty == 0 {
              break;
            }

            if let Some(resting_order) = self.orders.get_mut(&resting_id) {
              let fill_qty = std::cmp::min(remaining_qty, resting_order.quantity);
              remaining_qty -= fill_qty;
              resting_order.quantity -= fill_qty;

              if resting_order.quantity == 0 {
                queue.pop_front();
                self.orders.remove(&resting_id);
              }
            } else {
              queue.pop_front(); // Clear ghost ID
            }
          }
          if queue.is_empty() {
            prices_to_remove.push(bid_price);
          }
        }
        for price in prices_to_remove {
          self.bids.remove(&price);
        }
      }
    }
    remaining_qty
  }
}
