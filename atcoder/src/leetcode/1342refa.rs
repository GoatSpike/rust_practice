impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut n = num;
        let mut count = 0;

        while n != 0 {
            match n % 2 {
                0 => n /= 2,
                _ => n -= 1,
            }
            count += 1;
        }

        count
        // let mut n = num;
    }
}
"""
リファクタリングのポイント：
1.  変数 count の型指定を省略。
    Rust は型推論が強力なので、ここでは明示的な型指定を省略しました。
2.  イフブロック内で count のインクリメントを統一。
    count の増加は操作が完了した後の共通操作なので、ループの末尾に移動。
さらに、Rustでは match 式を使ってコードをもう少しエレガントにすることができます：
"""
