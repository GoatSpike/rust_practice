use std::collections::HashMap; // HashMapを使用するためのインポート

fn main() {
    let ransom_note = "aa".to_string(); // 身代金のメモの文字列
    let magazine = "aab".to_string(); // 雑誌の文字列
    println!("{}", can_construct(ransom_note, magazine)); // can_construct関数の結果を表示
}

// ransom_noteがmagazineから構成できるかどうかを判定する関数
fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map = HashMap::new(); // 文字の出現回数を記録するためのHashMapを作成
    for c in magazine.chars() { // 雑誌の各文字についてループ
        let current_count = map.get(&c).unwrap_or(&0); // 現在の文字の出現回数を取得（存在しない場合は0）
        map.insert(c, current_count + 1); // 出現回数を1増やしてHashMapに挿入
    }

    for c in ransom_note.chars() { // 身代金のメモの各文字についてループ
        let current_count = map.get(&c).unwrap_or(&0); // 現在の文字の出現回数を取得（存在しない場合は0）

        if *current_count == &0 { // 出現回数が0の場合はfalseを返す
            return false;
        } else {
            map.insert(c, current_count - 1); // 出現回数を1減らしてHashMapに挿入
        }
    }

    true // 全ての文字が条件を満たした場合はtrueを返す
}
