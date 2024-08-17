mod oracle;

use clap::Parser;
use oracle::Oracle;

#[derive(Parser)]
#[clap(
    name = "f8",
    version,
    author = "Lucas Vieira",
    about = "Ask a question and get an answer."
)]
struct Cli {
    #[clap(required = true)]
    question: Vec<String>,
}

fn main() {
    Cli::parse();
    let oracle = Oracle::new();
    let answer = oracle.answer();
    println!("{}", answer);
}
