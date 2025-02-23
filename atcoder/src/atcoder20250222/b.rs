use proconio::input;

fn main() {
    // 入力を受け取る
    input! {
        n: usize,
        mut s: [String; n],
    }

    // 文字列を長さの昇順にソート
    s.sort_by_key(|x| x.len());

    // ソートされた文字列を順に結合
    let result = s.concat();

    // 結果を出力
    println!("{}", result);
}
