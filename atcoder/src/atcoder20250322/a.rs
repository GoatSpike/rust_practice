use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut result: String = "".to_string();
  if n % 2 == 0 {
    let mid = n / 2;
    for i in 0..n {
      if i == mid - 1 || i == mid {
        result.push_str("=");
      } else {
        result.push_str("-");
      }
    }
  } else {
    let mid = n / 2;
    for i in 0..n {
      if i == mid {
        result.push_str("=");
      } else {
        result.push_str("-");
      }
    }
  }
  println!("{}", result);
}
