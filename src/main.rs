fn main() {
    let s = "Hello, World!";

    let mut s1 = String::from(s);
    s1.push_str(s);

    let s2 = &s1;

    println!("{s}");
    println!("{s1}");
    println!("{s2}");

}
