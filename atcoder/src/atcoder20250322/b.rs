use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    cards: [i32; 7],
  }

  if cards.len() != 7 {
    println!("No");
    return;
  }

  let mut hash_map = HashMap::new();
  for card in cards {
    let current_count = hash_map.get(&card).unwrap_or(&0);
    hash_map.insert(card, current_count + 1);
  }

  let mut two_card = false;
  let mut three_card = false;
  hash_map.iter().for_each(|(_key, value)| {
    if !two_card {
      two_card = if *value % 2 == 0 { true } else { false };
    }
    if !three_card {
      three_card = if *value % 3 == 0 { true } else { false };
    }
  });

  let result = if two_card && three_card { "Yes" } else { "No" };
  println!("{}", result);
}
