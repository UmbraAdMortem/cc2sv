use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "ccsv",
    version,
    about = "Conventional Commits 2 Semantic Versions",
    long_about = "Conventional Commits 2 Semantic Versions\n\nA language agnostic CLI tool written in rust for generating semantic versions and change-logs by parsing conventional commits."
)]
struct Args {
    /// The name to greet
    #[arg(short, long, default_value = "World")]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}
