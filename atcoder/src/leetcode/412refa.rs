fn main() {
    let n = 15;
    // This is the same as the snippet from atcoder/src/leetcode/412.rs
    let results: Vec<String> = (1..=n)
        .map(|i| {
            if i % 3 == 0 && i % 5 == 0 {
                "FizzBuzz".to_string()
            } else if i % 3 == 0 {
                "Fizz".to_string()
            } else if i % 5 == 0 {
                "Buzz".to_string()
            } else {
                i.to_string()
            }
        })
        .collect();
      //

    results
}
