use proconio::input;

fn main() {
    input! {
        n: f32,
    }

    if n >= 38.0 {
        println!("1");
    } else if n >= 37.5 && n < 38.0 {
        println!("2");
    } else {
        println!("3");
    }
}
