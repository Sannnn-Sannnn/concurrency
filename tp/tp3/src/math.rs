pub fn leibniz (n: u64) -> f64 {
  let mut output = 0.0;
  for i in 1..n + 1 {
    output += sign(i) / (2 * i + 1) as f64;
  }
  output
}

fn sign(n: u64) -> f64 {
  if n % 2 == 0 { 1.0 } else { -1.0 }
}