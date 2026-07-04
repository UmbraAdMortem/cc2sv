fn main() {
    let s = "Hello, World!";

    let mut s1 = String::from(s);
    s1.push_str(s);

    let s2 = &s1;

    println!("{s}");
    println!("{s1}");
    println!("{s2}");

    let word_end = length_first_word(&s1);
    println!("{word_end}");
    let slice1 = w0(&s1);

    let (b, e) = second_word(&s1);
    println!("{b} {e}");
    let slice2 = w1(&s1);

    println_substring(&s1, 0, word_end);
    println!("{slice1}");
    println_substring(&s1, b, e);
    println!("{slice2}");

    s1.clear();

    println_substring(&s1, 0, word_end);
    // println!("{slice1}");
    println_substring(&s1, b, e);
    // println!("{slice2}");

    // assert_eq!(slice2, &[7, 5]);

    let a = [1, 2, 3, 4, 5];
    println!("{:?}", &a);
    let slice_a = &a[1..3];
    println!("{:?}", &slice_a);
    // let slice_b = &b[2, 3];

    assert_eq!(slice_a, &[2, 3]);
}

fn length_first_word(s: &str) -> usize {
    for (p, c) in s.chars().enumerate() {
        if !c.is_ascii_alphanumeric() {
            return p;
        }
    }

    s.len()
}
fn w0(s: &str) -> &str {
    for (p, c) in s.chars().enumerate() {
        if !c.is_ascii_alphanumeric() {
            return &s[..p];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> (usize, usize) {
    let mut b = 0usize.wrapping_sub(1);
    let mut e = s.len();
    for (p, c) in s.chars().enumerate() {
        if c.is_whitespace() && p < b {
            b = p + 1;
        } else if !c.is_ascii_alphanumeric() && p >= b {
            e = p;
            break;
        }
    }

    (b, e)
}

fn w1(s: &str) -> &str {
    let mut b = 0usize.wrapping_sub(1);
    let mut e = s.len();
    for (p, c) in s.chars().enumerate() {
        if c.is_whitespace() && p < b {
            b = p + 1;
        } else if !c.is_ascii_alphanumeric() && p >= b {
            e = p;
            break;
        }
    }

    &s[b..e]
}

fn println_substring(s: &str, b: usize, e: usize) {
    for (p, c) in s.chars().enumerate() {
        if p >= e {
            break;
        }
        if p >= b {
            print!("{c}");
        }
    }
    print!("\n");
}
