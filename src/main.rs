mod benchmark;

use benchmark::Export;
use clap::{Parser, Subcommand};
use csv::Writer;
use std::io::Write;

use rand::rng;
use rand::seq::SliceRandom;

#[derive(Subcommand, Debug, Clone)]
enum Mode {
    /// Run the benchmarks
    Run {
        /// Number of runs per task
        #[arg(short, long, default_value_t = 1)]
        runs: u64,
        /// Run benchmarks in order
        #[arg(short, long, action)]
        ordered: bool,
        /// How many seconds delay between each benchmark
        #[arg(short,long, default_value_t = 0)]
        cooldown: u64
    },
    /// Compile the benchmarks
    Compile,
}

#[derive(Debug, Parser)]
#[command(version, verbatim_doc_comment)]
struct CLI {
    #[command(subcommand)]
    mode: Mode,
    /// Set path to benchmarks directory
    #[arg(short, long, default_value = "./benchmarks")]
    path: String,
    /// Set language filter
    #[arg(short, long)]
    language: Option<String>,
    /// Set task filter
    #[arg(short, long)]
    task: Option<String>,
}

fn main() {
    let args = CLI::parse();

    let mut tasks = match benchmark::list_all(args.path) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Filter out unwanted tasks
    tasks = filter_list(tasks, args.language.as_deref(), args.task.as_deref());

    let str = match args.mode {
        Mode::Run{ .. } => "Running",
        Mode::Compile => "Compiling"
    };

    println!("{} {} benchmarks ...", str, tasks.len());

    match args.mode {
        Mode::Run { runs, ordered, cooldown } => run_and_export(tasks, runs, ordered, cooldown),
        Mode::Compile => benchmark::compile(tasks),
    }
}

fn filter_list(tasks: Vec<benchmark::Task>, lang: Option<&str>, name: Option<&str>) -> Vec<benchmark::Task> {
  
    tasks.into_iter()
        .filter(|t| lang.is_none() || t.language.to_lowercase() == lang.unwrap().to_lowercase())
        .filter(|t| name.is_none() || t.name.to_lowercase() == name.unwrap().to_lowercase())
        .collect()
}

fn run_and_export(unique_tasks: Vec<benchmark::Task>, runs: u64, ordered: bool, cooldown: u64) {
    
    let mut tasks = vec![];
    for ut in unique_tasks {
        for _ in 0..runs {
            tasks.push(ut.clone());
        }
    }

    if !ordered {
        tasks.shuffle(&mut rng());
    }

    let Ok(exports) = benchmark::run(tasks, runs, cooldown) else { panic!("AAAA") };
    let _ = csv(exports);
}

use std::fs::File;
fn csv(data: Vec<Export>) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize to CSV
    let mut writer = Writer::from_writer(vec![]);
    for itt in data {
        writer.serialize(itt)?;
    }

    // Write data to CSV
    let data = String::from_utf8(writer.into_inner()?)?;
    let mut file = File::create("energy.csv")?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
