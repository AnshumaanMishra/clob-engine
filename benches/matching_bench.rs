use clob_engine::engine::OrderBook;
use clob_engine::types::{Order, Side};
use hdrhistogram::Histogram;
use std::hint::black_box;
use std::time::Instant;

fn main() {
  let mut hist = Histogram::<u64>::new_with_bounds(1, 10_000_000_000, 3).unwrap();
  let iterations = 100_000;

  println!("Running {} iterations:", iterations);

  for _ in 0..iterations {
    let mut book = OrderBook::new();
    for i in 1..=10000 {
      book.add_limit_order(Order {
        id: i,
        side: Side::Sell,
        price: 100 + (i % 50),
        quantity: 10,
      });
    }

    let aggressive_buy = Order {
      id: 99999,
      side: Side::Buy,
      price: 150,
      quantity: 500,
    };

    let start = Instant::now();
    book.add_limit_order(black_box(aggressive_buy));
    let elapsed = start.elapsed().as_nanos() as u64;

    hist.record(elapsed).unwrap();

    drop(book);
  }

  println!("\n Latency Results (ns) ");
  println!("Mean:    {:>8} ns", hist.mean() as u64);
  println!("Min:     {:>8} ns", hist.min());
  println!("p50:     {:>8} ns", hist.value_at_quantile(0.50));
  println!("p90:     {:>8} ns", hist.value_at_quantile(0.90));
  println!("p99:     {:>8} ns", hist.value_at_quantile(0.99));
  println!("p99.9:   {:>8} ns", hist.value_at_quantile(0.999));
  println!("p99.99:  {:>8} ns", hist.value_at_quantile(0.9999));
  println!("Max:     {:>8} ns", hist.max());
}
