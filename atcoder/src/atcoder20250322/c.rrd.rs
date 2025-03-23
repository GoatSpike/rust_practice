use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    numbers: [usize; n],
  }

  let mut d: HashMap<usize, Vec<usize>> = HashMap::new();

  // Populate the hashmap with list indices
  for (i, &value) in numbers.iter().enumerate() {
    d.entry(value).or_insert(Vec::new()).push(i);
  }

  // Sort keys in descending order and check lengths of value lists
  let mut keys: Vec<&usize> = d.keys().collect();
  keys.sort_by(|a, b| b.cmp(a));

  for &key in keys {
    if d[&key].len() == 1 {
      println!("{}", d[&key][0] + 1);
      return;
    }
  }

  println!("-1");
}
