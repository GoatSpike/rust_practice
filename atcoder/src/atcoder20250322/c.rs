use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    numbers: [i64; n],
  }

  let mut number_hash = HashMap::new();
  for number in numbers {
    let current_count = numbers.get(&number).unwrap_or(&0);
    number_hash.insert(number, current_count + 1);
  }

  let mut max_count = 0;
  let mut max_number = 0;
  for (number, count) in number_hash {
    if *count > max_count {
      max_count = *count;
      max_number = *number;
    }
  }

  numbers.retain(|&x| x != max_number);
  println!("{:?}", numbers);
}
