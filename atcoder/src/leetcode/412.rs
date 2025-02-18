fn main() {
  let n = 15;
  let mut results = Vec::new();
    for i in 1..n+1 {
          if i % 3 == 0 && i % 5 == 0 {
          results.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            results.push("Fizz".to_string());
        } else if i % 5 == 0 {
            results.push("Buzz".to_string());
        } else {
            results.push(i.to_string());
        }
    }
  results
}
