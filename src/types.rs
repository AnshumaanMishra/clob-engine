pub type OrderId = u64;
pub type Price = u64;
pub type Quantity = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
  Buy,
  Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
  pub id: OrderId,
  pub side: Side,
  pub price: Price,
  pub quantity: Quantity,
}
