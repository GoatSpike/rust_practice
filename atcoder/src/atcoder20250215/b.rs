use proconio::input;

fn main() {
    // 標準入力から文字列を読み込む
    input! {
        s: String
    }

    let n = s.len();
    let chars: Vec<char> = s.chars().collect();
    let mut count = 0;

    // 文字列を走査
    for i in 0..n {
        if chars[i] == 'A' {
            // iの後ろ全ての位置を見ていく
            for j in i+1..n {
                if chars[j] == 'B' {
                    let distance = j - i;
                    let k = j + distance;
                    // kが文字列の範囲内かつ chars[k] が 'C' かどうかチェック
                    if k < n && chars[k] == 'C' {
                        count += 1;
                    }
                }
            }
        }
    }

    // 条件を満たす組の数を出力
    println!("{}", count);
}
