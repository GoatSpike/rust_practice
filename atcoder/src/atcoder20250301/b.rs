use proconio::input;

fn main() {
    // 標準入力からNを受け取る
    input! {
        n: usize,
    }

    // グリッドを初期化（すべてのマスを色なしとする）
    let mut grid = vec![vec!['.'; n]; n];

    // Nの行に沿って操作を行う
    for i in 1..=n {
        let j = n + 1 - i; // jを計算

        // iがj以下でない場合何もしない
        if i <= j {
            // 黒として塗るか、白として塗るかを決定
            let color = if i % 2 == 1 { '#' } else { '.' };

            // 固定された範囲内のマスを塗りつぶす
            for x in i..=j { // 行ループ
                for y in i..=j { // 列ループ
                    grid[x - 1][y - 1] = color;
                }
            }
        }
    }

    // 結果の出力
    for row in grid {
        println!("{}", row.into_iter().collect::<String>());
    }
}
