use proconio::input;

fn main() {
  input!{
    s1: String,
    s2: String,
  }

  if s1 == "sick" && s2 == "sick" {
    println!("1");
  } else if s1 == "sick" && s2 == "fine" {
    println!("2");
  } else if s1 =="fine" && s2 == "sick" {
    println!("3");
  } else {
    println!("4");
  }
}
