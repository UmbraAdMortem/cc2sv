use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;

mod cli;

pub struct CCSV {
    args: Vec<String>,
}

impl CCSV {
    pub fn new() -> Self {
        return CCSV::new_internal(std::env::args().collect());
    }
}

impl CCSV {
    fn new_internal(args: Vec<String>) -> Self {
        CCSV { args }
    }

    pub fn run(&mut self) -> Result<()> {
        // https://docs.rs/clap_builder/4.6.7/src/clap_builder/derive.rs.html#52-55
        let cli = Cli::try_parse_from(self.args.iter()).inspect_err(|e| {
            // https://docs.rs/clap_builder/4.6.7/src/clap_builder/error/mod.rs.html#245
            let _ = e.print();
        });

        println!("\nArgs:{:#?}", self.args);
        println!("Hello! \nThis is Conventional Commits 2 Sematic Verison!");
        eprintln!("There are no errors!");

        cli?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gag::BufferRedirect;
    use std::io::Read;

    // https://rust-cli.github.io/book/tutorial/testing.html
    #[test]
    fn test_stdout_not_empty() {
        let stdout = BufferRedirect::stdout().unwrap();
        let stderr = BufferRedirect::stderr().unwrap();

        assert!(
            CCSV::new_internal(args(&["target/debug/ccsv"]))
                .run()
                .is_ok()
        );

        let mut out = String::new();
        let _ = stdout.into_inner().read_to_string(&mut out);
        println!("out: {}", out);
        assert!(!out.is_empty());

        let mut err = String::new();
        let _ = stderr.into_inner().read_to_string(&mut err);
        println!("err: {}", err);
        assert!(!err.is_empty());
    }

    #[test]
    fn test_help() {
        let stdout = BufferRedirect::stdout().unwrap();
        let stderr = BufferRedirect::stderr().unwrap();

        assert!(
            CCSV::new_internal(args(&["target/debug/ccsv", "--help"]))
                .run()
                .is_err()
        );

        let mut out = String::new();
        let _ = stdout.into_inner().read_to_string(&mut out);
        println!("out: {}", out);
        assert!(out.starts_with("Usage:"));

        let mut err = String::new();
        let _ = stderr.into_inner().read_to_string(&mut err);
        println!("err: {}", err);
        assert!(!err.is_empty());
    }

    fn args(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }
}
