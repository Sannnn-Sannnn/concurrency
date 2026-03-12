use std::time::Duration;
use std::thread;
const DEBUG: bool = false;
pub fn leibniz (n: u64) -> f64 {
  if DEBUG {
    for _ in 0..n {
      thread::sleep(Duration::new(1, 0));
    }
  }
  n as f64 / (2 * n + 1) as f64
}