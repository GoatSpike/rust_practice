use proconio::input;
use std::collections::HashSet;

fn main() {
    // 入力の読み込み
    input! {
        _n: usize,  // N は使わないので、変数名として _n に変更
        m: usize,   // M を snake_case に変更
        // edges という名前のタプルのベクターを読み込んでいます。
        // 各タプルは2つの usize 型の値を持ち、m 個のタプルが含まれています。
        edges: [(usize, usize); m],
    }

    // 自己ループのカウント
    let mut self_loops = 0;
    let mut unique_edges = HashSet::new();

    for &(u, v) in &edges {
        if u == v {
            self_loops += 1;
        } else {
            // 頂点の順序に依存しないようにソートされたタプルをセットに追加
            let edge = if u < v { (u, v) } else { (v, u) };
            unique_edges.insert(edge);
        }
    }

    // 多重辺のカウント
    let mut m_with_out_self_loops = m - self_loops;
    let multiple_edges = m_with_out_self_loops - unique_edges.len();

    // 取り除く必要のある辺の合計
    let edges_to_remove = self_loops + multiple_edges;
    println!("{}", edges_to_remove);
}
