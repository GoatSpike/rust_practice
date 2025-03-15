use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        numbers: [i32; n],
    }
    let mut prefix_unique_count = vec![0; n + 1];
    let mut seen = HashSet::new();

    for i in 0..n {
        seen.insert(numbers[i]);
        prefix_unique_count[i + 1] = seen.len();
    }
    let mut suffix_unique_count = vec![0; n + 1];
    seen.clear();

    for i in (0..n).rev() {
        seen.insert(numbers[i]);
        suffix_unique_count[i] = seen.len();
    }

    let mut result = 0;
    for i in 0..n {
        result = result.max(prefix_unique_count[i] + suffix_unique_count[i]);
    }

    println!("{}", result);
}
