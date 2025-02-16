use proconio::input;

// GCDを求める関数
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }
    let mut results = vec![0; n];
    // 各a[i]について
    for i in 0..n {
        let ai = a[i];
        let mut multiples = Vec::new();
        // aからiを含んだ辺りのk-1個を選ぶ
        for &aj in a.iter() {
            let g = gcd(ai, aj);
            multiples.push(g);
        }
        // k個のGCDを満たす組み合わせを見つける
        multiples.sort_unstable_by(|a, b| b.cmp(a));  // 降順にソート
        let mut current_gcd = multiples[0];
        for m in 1..k {
            current_gcd = gcd(current_gcd, multiples[m]);
        }
        results[i] = current_gcd;
    }
    // 結果の出力
    for result in results {
        println!("{}", result);
    }
}
