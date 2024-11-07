mod benchmark;

use clap::Parser;

use benchmark::benchmark;

#[derive(Debug, Parser)]
#[command(version, verbatim_doc_comment)]
struct CLI {
    #[arg(short, long, default_value_t = 11)]
    runs: u64,
}

fn main() {
    let args = CLI::parse();

    println!("{} µj", benchmark("pwd", args.runs));
    println!("{} µj", benchmark("", args.runs));
}
