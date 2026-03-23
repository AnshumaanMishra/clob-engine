use clob_engine::engine::OrderBook;
use clob_engine::types::{Order, Side};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_matching_engine(c: &mut Criterion) {
  c.bench_function("match_order", |b| {
    b.iter_batched(
      || {
        // Setup: Pre-populate the book with 10,000 resting sell orders
        let mut book = OrderBook::new();
        for i in 1..=10000 {
          book.add_limit_order(Order {
            id: i,
            side: Side::Sell,
            price: 100 + (i % 50), // Spread prices between 100 and 150
            quantity: 10,
          });
        }
        // The aggressive buy order that will sweep multiple levels
        let aggressive_buy = Order {
          id: 99999,
          side: Side::Buy,
          price: 150,
          quantity: 500,
        };
        (book, aggressive_buy)
      },
      |(mut book, order)| {
        // The actual hot-path being measured
        book.add_limit_order(black_box(order));
      },
      criterion::BatchSize::SmallInput,
    )
  });
}

criterion_group!(benches, bench_matching_engine);
criterion_main!(benches);
