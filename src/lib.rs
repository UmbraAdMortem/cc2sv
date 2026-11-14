//! Conventional Commits to Semantic Versioning library
//!
//! Provides a simple interface which parses CLI arguments, calculates the current semantic version of a provided directory by analysing its converional commit history and can also generate the corresponding changelog.
//!

// #![warn(missing_docs)]
#![deny(missing_docs)]

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;

mod cli;

/// Public intervate of the `ccsv` libreary package
pub struct CCSV {
    args: Vec<String>,
}

impl CCSV {
    /// Creates a new instance of the `CCSV` interface
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ccsv::CCSV;
    /// fn main() {
    ///     CCSV::new(std::env::args().collect()).run();
    /// }
    /// ```
    pub fn new(args: Vec<String>) -> Self {
        CCSV { args }
    }

    /// Runs `ccsv`'s functionality according to CLI arguments and configuration
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ccsv::CCSV;
    /// fn main() {
    ///     CCSV::new(std::env::args().collect()).run();
    /// }
    /// ```
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
