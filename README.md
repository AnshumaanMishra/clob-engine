# clob-engine

A price-time priority order book written in Rust. Supports limit orders and cancels,
with a matching engine that walks the opposing side on each aggressive order.

Built to understand the core data structure at the heart of every exchange and
market maker's stack.

---

## How it works

The book keeps two sides: bids sorted highest-first, asks sorted lowest-first.
Each price level holds a queue of order IDs in arrival order (time priority).
A separate `FxHashMap` stores the full order data keyed by ID, so cancel is O(1)
lookup without touching the price-level queues.

When a new limit order arrives, the matching loop walks the opposing side and
fills against resting orders until either the aggressor is fully filled or no
more prices cross. Whatever is left rests on the book.

One known tradeoff: cancel only removes the order from the hashmap. The order ID
stays in the price-level queue until the matching loop encounters it and skips it
(the "ghost ID" path). This avoids an O(n) queue scan on every cancel at the cost
of slightly more work during matching on a heavily-cancelled book.

## Benchmark

The bench pre-populates 10,000 resting sell orders across 50 price levels, then
sends in a single aggressive buy that sweeps through multiple levels.

```
match_order   time: [3.9315 us  4.0528 us  4.1543 us]
```

Run it yourself:

```bash
cargo bench
```

## Usage

```rust
use clob_engine::engine::OrderBook;
use clob_engine::types::{Order, Side};

let mut book = OrderBook::new();

book.add_limit_order(Order { id: 1, side: Side::Sell, price: 101, quantity: 50 });
book.add_limit_order(Order { id: 2, side: Side::Sell, price: 102, quantity: 50 });

// this crosses the spread and fills against id=1
book.add_limit_order(Order { id: 3, side: Side::Buy, price: 101, quantity: 30 });

book.cancel_order(2);
```

## Structure

```
src/
  types.rs    -- Order, Side, type aliases for Price/Quantity/OrderId
  engine.rs   -- OrderBook struct and matching logic
  lib.rs      -- module exports
benches/
  matching_bench.rs  -- Criterion bench, multi-level sweep scenario
```

## Dependencies

- `rustc-hash` for `FxHashMap`: non-cryptographic hasher, roughly 2x faster than
  std on integer keys which is what order IDs are
- `criterion` (dev) for benchmarking
