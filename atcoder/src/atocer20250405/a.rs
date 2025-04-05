use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut result: isize = 0;
  let check = 400 % n;
  if check == 0 {
    result = 400 / n;
  } else {
    result = -1;
  }

  println!("{}", result);

}
