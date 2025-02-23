use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() - 1 {
        if chars[i] == 'W' && chars[i + 1] == 'A' {
            chars[i] = 'A';
            chars[i + 1] = 'C';
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    let result: String = chars.into_iter().collect();
    println!("{}", result);
}
