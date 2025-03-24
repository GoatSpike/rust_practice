use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let result = generate_pattern(n);
  println!("{}", result);
}

fn generate_pattern(n: usize) -> String {
  let mut result = String::new();
  let mid = n / 2;

  for i in 0..n {
    if n % 2 == 0 && (i == mid - 1 || i == mid) {
      result.push('=');
    } else if n % 2 != 0 && i == mid {
      result.push('=');
    } else {
      result.push('-');
    }
  }

  result
}
