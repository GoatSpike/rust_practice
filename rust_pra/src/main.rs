fn main() {
    println!("Hello, world!");

    sec3();

    sec4();
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
