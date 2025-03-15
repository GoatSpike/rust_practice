use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut result = 0;
    let mut chars = s.chars().peekable();
    if let Some(&first_c) = chars.peek() {
        if first_c == 'o' {
            result += 1;
        }
    }

    while let Some(c) = chars.next() {
        if c == 'i' {
            if let Some(&next_c) = chars.peek() {
                if next_c == 'i' {
                    result += 1;
                }
            } else {
                result += 1;
            }
        } else if c == 'o' {
            if let Some(&next_c) = chars.peek() {
                if next_c == 'o' {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}
