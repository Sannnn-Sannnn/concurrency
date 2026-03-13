use std::time::Duration;
use std::thread;

const DEBUG: bool = false;
// When true, each request will take i seconds to be processed.
// This way, it will be possible to test concurrency.
// Otherwise, Rust is too fast to test for big values.

pub fn leibniz (n: u64) -> f64 {
  if DEBUG {
    for _ in 0..n {
      thread::sleep(Duration::new(1, 0));
    }
  }
  let mut output = 0.0;
  for i in 1..n + 1 {
    output += is_even(i) / (2 * i + 1) as f64;
  }
  output
}

fn is_even(n: u64) -> f64 {
  if n % 2 == 0 { 1.0 } else { -1.0 }
}