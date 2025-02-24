use proconio::input;

fn main() {
    // 入力を受け取る
    input! {
        input: String
    }

    // '2' だけを抽出して結果を構築
    let result: String = input.chars().filter(|&c| c == '2').collect();

    // 結果を出力
    println!("{}", result);
}
