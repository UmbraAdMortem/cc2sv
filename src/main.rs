//! Conventional Commits to Semantic Versioning exeuteable
//!
//! Provides a simple CLI interface to calculate the current semantic version of a provided directory by analysing its converional commit history and can also generate the corresponding changelog.
//!

// #![warn(missing_docs)]
#![deny(missing_docs)]

use anyhow::Result;
use ccsv::CCSV;

fn main() -> Result<()> {
    CCSV::new(std::env::args().collect()).run()
}
