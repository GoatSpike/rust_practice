use std::env;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
        .expect("error parsing argument"));
        }

        if numbers.len() == 0 {
            eprintln!("Usage: gcd NUMBER ...");
            std::process::exit(1);
        }
        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = gcd(d, *m);
        }


    println!("The greatest common divisor of {:?} is {}", numbers, d);

    // println!("Hello, world!");

    // sec3();

    // sec4();

    // sec5();

    // sec6();

    let s = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let t = s;
    let u = s;

    type Table = HashMap<String, Vec<String>>;
    let mut table = Table::new();

    table.insert("Dazai".to_string(),
                 vec!["ningen shikkaku".to_string(), "unknown".to_string()]);

    table.insert("Chuuya".to_string(),
                 vec!["dead apple".to_string(), "unknown".to_string()]);

    table.insert("Akutagawa".to_string(),
                 vec!["rashoumon".to_string(), "unknown".to_string()]);

    show(&table);
}

fn show(table: &Table) {
    for (key, Value) in table {
        println!("works by {}", key);
        for v in Value {
            println!("  {}", v);
        }
    }
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
    if m < n {
    let t = m;
    m = n;
    n = t;
    }
    m = m % n;
    }
    n
}

fn sec3() {
    let tup: (i32, u64, f32) = (500, 500, 500.0);

    println!("tup: {:?}", tup);

    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    println!("tup1: {}, tup2: {}, tup3: {}", tup.0, tup.1, tup.2);

    let array1 = [1, 2, 3, 4, 5];
    println!("array1: {:?}", array1);

    let array2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array2: {:?}", array2);

    let array3 = [3; 5];
    println!("array3: {:?}", array3);

    let plus_one = plus_one(5);
    println!("plus_one: {}", plus_one);

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);

    let mut count = 0;
    'counting: loop {
        println!("count: {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("count: {}", count);

    let numbers = [10, 20, 30, 40, 50];

    for ele in numbers.iter() {
        println!("ele: {}", ele);
    }

    for ele in (numbers[1]..numbers[4]).rev() {
        println!("ele: {}", ele);
        // 50 ~ 20 DESC
    }

    for number in (1..4).rev() {
        println!("number: {}", number);
        // 3 ~ 1 DESC
    }
}

fn plus_one(s: i32) -> i32 {
    s + 1
}


fn sec4() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("s: {}", s);

    let s1 = String::from("value1");
    let s2 = s1;

    println!("s1: {}, world!", s2);

    let s3 = String::from("value3");
    let s4 = s3.clone();

    println!("s3: {}!, s4: {}", s3, s4);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



fn sec5() {
    let mut user1 = User {
        email: String::from("someone@ex.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("change@ex.com");
    println!("user1 {:?}", user1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1: {:?}", rect1);
    println!("rect1 area: {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn sec6() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:?}, six: {:?}", four, six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home: {:?}, loopback: {:?}", home, loopback);
}
