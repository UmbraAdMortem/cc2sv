// Exits function on error
// use anyhow::{Context, Ok, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "ccsv",
    version,
    about = "Conventional Commits 2 Semantic Versions",
    long_about = "Conventional Commits 2 Semantic Versions\n\nA language agnostic CLI tool written in rust for generating semantic versions and change-logs by parsing conventional commits."
)]
struct Args {
    // The name to greet
    #[arg(short, long, default_value = "World")]
    name: String,

    // Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    // Set working directory
    #[arg(short, long, default_value = ".")]
    workdir: std::path::PathBuf,
}

// Exits function on error
// fn main() -> Result<(), Box<dyn std::error::Error>> {
// Continues function on error
fn main() {
    // TODO: Refactor cli  handling
    let args = Args::parse();
    dbg!(&args);

    dbg!(&std::env::current_dir().expect("Current dir should be valid"));
    dbg!(&args.workdir);

    // Exits function on error
    // std::env::set_current_dir(&args.workdir)
    //     .with_context(|| format!("\nMissing dir: {}", &args.workdir.display()))?;
    // println!("Changed dir: {}", &args.workdir.display());

    // Continues function on error
    match std::env::set_current_dir(&args.workdir) {
        Ok(_) => {
            dbg!(&std::env::current_dir().expect("Current dir should be valid"));
        }
        Err(_) => {
            // TODO: Implement: mkdir -p and surround set_current_dir with loop
            eprintln!("Missing dir: {:#?}", &args.workdir);
        }
    }

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }

    // Exits function on error
    // Ok(())
}
