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
// match文を使うことで、コードをより簡潔にすることができます。
//   let result = match (s1.as_str(), s2.as_str()) {
//     ("sick", "sick") => 1,
//     ("sick", "fine") => 2,
//     ("fine", "sick") => 3,
//     _ => 4,
//   };
// println!("{}", result);
}
