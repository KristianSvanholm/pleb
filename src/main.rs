mod benchmark;

use clap::Parser;
use std::process::Command;

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

    println!("{} µj", benchmark(generate_command(&args.command), args.runs));
}

fn generate_command(command: &str) -> Command {
    let parts: Vec<&str> = command.split(" ").collect();
    let mut cmd = Command::new(parts[0]);

    for part in parts.into_iter().skip(1) {
       cmd.arg(part);
    }

    cmd
}
