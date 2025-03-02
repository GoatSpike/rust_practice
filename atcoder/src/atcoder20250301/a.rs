use proconio::input;

fn main() {
    input! {
        n: usize,
        numbers: [i32; n],
    }

    for i in 0..n-1 {
        if numbers[i] >= numbers[i+1] {
          println!("No");
          return;
        }
    }
    println!("Yes");
}
