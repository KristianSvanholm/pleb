mod benchmark;

use benchmark::Exports;
use clap::Parser;
use std::{io::Write, process::Command};
use csv::Writer;

#[derive(Debug, Parser)]
#[command(version, verbatim_doc_comment)]
struct CLI {
    #[arg(short, long, default_value_t = 11)]
    runs: u64,
}

fn main() {
    let args = CLI::parse();


    let exports = match benchmark::run("../../benchmarks", args.runs) {
        Ok(exp) => exp,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let _ = csv(exports);
}

use std::fs::File;
fn csv(data: Vec<Exports>) -> Result<(), Box<dyn std::error::Error>> {

    // Serialize to CSV
    let mut writer = Writer::from_writer(vec![]);
    for lang in data {
        for itt in lang.0 {
            writer.serialize(itt)?;
        }
    }

    // Write data to CSV
    let data = String::from_utf8(writer.into_inner()?)?;
    let mut file = File::create("energy.csv")?;
    file.write_all(data.as_bytes())?;
   
    Ok(())
}

fn generate_command(command: &str) -> Command {
    let parts: Vec<&str> = command.split(" ").collect();
    let mut cmd = Command::new(parts[0]);

    for part in parts.into_iter().skip(1) {
        cmd.arg(part);
    }

    cmd
}
