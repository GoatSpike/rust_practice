use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    numbers: [usize; n],
  }

  // Create a hashmap to store the indices of the numbers
  let mut d: HashMap<usize, Vec<usize>> = HashMap::new();

  // Populate the hashmap with list indices
  for (i, &value) in numbers.iter().enumerate() {
    d.entry(value).or_insert(Vec::new()).push(i);
  }

  // Sort keys in descending order and check lengths of value lists
  let mut keys: Vec<&usize> = d.keys().collect();
  keys.sort_by(|a, b| b.cmp(a));

  // Find the first key with a value list of length 1
  for &key in keys {
    if d[&key].len() == 1 {
      println!("{}", d[&key][0] + 1);
      return;
    }
  }

  println!("-1");
}
