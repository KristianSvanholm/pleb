mod linux;
mod macos;

#[cfg(target_os = "linux")]
use linux::benchmark::benchmark;
#[cfg(target_os = "macos")]
use macos::benchmark::benchmark;

use clap::Parser;

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
