use ccsv::CCSV;

// https://rust-cli.github.io/book/tutorial/testing.html

#[test]
fn test_stdout_not_empty() {
    let mut out: Vec<u8> = Vec::new();

    CCSV::new(&mut out).run();

    assert!(!out.is_empty());
    println!("{}", String::from_utf8(out).unwrap());
}
