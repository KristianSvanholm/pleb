mod benchmark;

use clap::Parser;

use benchmark::benchmark;

#[derive(Debug, Parser)]
#[command(version, verbatim_doc_comment)]
struct CLI {
    #[arg(short, long, default_value_t = 11)]
    runs: u64,
    #[arg(short, long, default_value = "pwd")]
    command: String,
}

fn main() {
    let args = CLI::parse();

    println!("{} µj", benchmark(&args.command, args.runs));
}
