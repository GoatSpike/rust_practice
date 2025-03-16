use std::io::{self, BufRead};

fn sol(a: i64, b: i64, c: i64) -> i64 {
    // ax^2 + bx + c = 0の解
    let mut l = 0;
    let mut r = 600000001;
    while r - l > 1 {
        let mid = (l + r) / 2;
        if a * mid * mid + b * mid + c <= 0 {
            l = mid;
        } else {
            r = mid;
        }
    }
    if a * l * l + b * l + c == 0 {
        return l;
    }
    -1
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let n: i64 = iterator.next().unwrap().unwrap().trim().parse().unwrap();

    for d in 1.. {
        if d * d * d > n {
            break;
        }
        // (k+d)^3 - k^3 = d^3 + 3*d^2*k + 3*d*k^2 = n
        if n % d != 0 {
            continue;
        }
        let m = n / d; // =3*k^2 + 3*dk + d^2
        let k = sol(3, 3 * d, d * d - m);
        if k > 0 {
            println!("{} {}", k + d, k);
            return;
        }
    }
    println!("-1");
}
