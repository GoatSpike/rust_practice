// use ferris_says::say; // from the previous sep
// use std::io::{stdout, BufWriter};

fn main() {
    // let stdout = stdout();
    // let message = "Hello, fellow Rustaceans!";
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(&message, width, &mut writer).unwrap();
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);

    sec2();
}

fn sec2() {
    let space = " ";
    let space = space.len();

    println!("space: {}", space);

    // let mut spaces = "    ";
    // spaces = spaces.len();
    // println!("spaces: {}", spaces);
}
