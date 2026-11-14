use ccsv::CCSV;
use gag::BufferRedirect;

use crate::common::{args, read_drop_buffer, redirect_stderr, redirect_stdout};

mod common;

// https://rust-cli.github.io/book/tutorial/testing.html
#[test]
fn test_no_args() {
    let stdout = redirect_stdout();
    let stderr = redirect_stderr();

    assert!(CCSV::new(args(&["target/debug/ccsv"])).run().is_ok());

    let out = read_drop_buffer(stdout);
    println!("out:\n{}", out);

    let err = read_drop_buffer(stderr);
    println!("err:\n{}", err);

    assert!(out.is_empty());

    assert!(err.contains("Usage"));
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
    println!("out:\n{}", out);

    let err = read_drop_buffer(stderr);
    println!("err:\n{}", err);

    assert!(out.contains("-h"));
    assert!(out.contains("--help"));

    assert!(err.is_empty());
}
