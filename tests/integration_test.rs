use ccsv::CCSV;
use gag::BufferRedirect;

use crate::common::{args, read_drop_buffer, redirect_stderr, redirect_stdout};

mod common;

// https://rust-cli.github.io/book/tutorial/testing.html
#[test]
fn test_stdout_not_empty() {
    let stdout = redirect_stdout();
    let stderr = redirect_stderr();

    assert!(CCSV::new(args(&["target/debug/ccsv"])).run().is_ok());

    let out = read_drop_buffer(stdout);
    println!("out: {}", out);
    assert!(!out.is_empty());

    let err = read_drop_buffer(stderr);
    println!("err: {}", err);
    assert!(!err.is_empty());
}

#[test]
fn test_help() {
    let stdout = BufferRedirect::stdout().unwrap();
    let stderr = BufferRedirect::stderr().unwrap();

    assert!(
        CCSV::new(args(&["target/debug/ccsv", "--help"]))
            .run()
            .is_err()
    );

    let out = read_drop_buffer(stdout);
    println!("out: {}", out);
    assert!(out.starts_with("Usage:"));

    let err = read_drop_buffer(stderr);
    println!("err: {}", err);
    assert!(!err.is_empty());
}
