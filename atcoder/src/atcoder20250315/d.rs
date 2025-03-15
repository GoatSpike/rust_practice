use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let limit = ((n.abs() as f64).cbrt().ceil() as i64) + 1;

    for x in -limit..=limit {
        let mut low = -limit;
        let mut high = limit;
        while low <= high {
            let mid = (low + high) / 2;
            let diff = x.pow(3) - mid.pow(3);
            if diff == n {
                println!("{} {}", x, mid);
                return;
            } else if diff < n {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }

    println!("-1");
}
