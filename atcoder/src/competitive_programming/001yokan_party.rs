use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if a[i] == a[j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
