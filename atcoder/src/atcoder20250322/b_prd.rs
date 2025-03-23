use proconio::input;

fn main() {
    let mut bk = vec![0; 13];

    for _ in 0..7 {
        input! {
            x: usize,
        }
        // Adjust for zero-indexing
        bk[x - 1] += 1;
    }

    // Sort in descending order
    bk.sort_by(|a, b| b.cmp(a));

    // Check the counts
    if bk[0] >= 3 && bk[1] >= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
