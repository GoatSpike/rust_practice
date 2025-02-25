use proconio::input;

fn main() {
    // 入力を受け取る
    input! {
        s: String
    }

    // 文字列を文字のベクタに変換
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    // 文字列を走査して特定のパターンを置換
    while i < chars.len() - 1 {
        // 現在の文字と次の文字が 'W' と 'A' である場合
        if chars[i] == 'W' && chars[i + 1] == 'A' {
            // 'W' を 'A' に置換
            chars[i] = 'A';
            // 'A' を 'C' に置換
            chars[i + 1] = 'C';
            // インデックスを1つ戻す（ただし、0未満にはならない）
            if i > 0 {
                i -= 1;
            }
        } else {
            // 条件に合わない場合は次の文字へ進む
            // インデックスを1つ進める
            i += 1;
        }
    }

    // 文字のベクタを再び文字列に変換
    let result: String = chars.into_iter().collect();
    // 結果を出力
    println!("{}", result);
}
