mod linux;
mod macos;

#[cfg(target_os = "linux")]
use linux::sampler::Sampler;
#[cfg(target_os = "macos")]
use macos::sampler::Sampler;

use chrono::{Duration, Utc};
use core::fmt;
use std::process::Command;
use std::fs::{self};
use std::io;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Export {
    language: String,
    task: String,
    duration: i64,
    energy: f64,
}

impl fmt::Display for Export {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} µj, {} ms, {:.5} w",
            self.energy,
            self.duration,
            self.energy / self.duration as f64
        )
    }
}

#[derive(Serialize)]
pub struct Exports(pub Vec<Export>);

impl fmt::Display for Exports {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Results:\n")?;
        for (i, v) in self.0.iter().enumerate() {
            write!(
                f,
                "{} - {} - {:>2} - {:>10} µj, {:>3} ms, {:>10.5} w\n",
                v.language,
                v.task,
                i,
                v.energy,
                v.duration,
                v.energy / v.duration as f64
            )?;
        }
        Ok(())
    }
}

pub fn run(path: &str, runs: u64) -> io::Result<Vec<Exports>> {
    let mut res: Vec<Exports> = vec![];

    // For each language
    for lang in fs::read_dir(path)? {
        let lang = lang?;
        let language_path = lang.path();

        // Skip files found
        if !language_path.is_dir() {
            continue;
        }

        // For each task
        for task in fs::read_dir(&language_path)? {
            let task = task?;
            let task_path = task.path();

            /*
                For a faster runtime
                if let Some(x) = task_path.to_str() {
                    if !x.contains("C") {
                        continue; 
                }
            }*/

            // Skip files found
            if !task_path.is_dir() {
                continue
            }

            if let Some(str) = task.path().to_str() {
                // Get language name and Task
                let parts: Vec<&str> = str.split("/").collect();

                // Create make command
                let mut cmd = Command::new("make");
                cmd.arg("-C").arg(str).arg("run");

                // Run benchmark
                res.push(benchmark(cmd, runs, parts[3],parts[4]));
            }
        }
    }

    Ok(res)
}

pub fn benchmark(mut cmd: Command, runs: u64, lang: &str, task: &str) -> Exports {
    let sampler = Sampler::new();

    let mut exports: Vec<Export> = vec![];

    for n in 0..runs {

        println!("Running {} / {} - {}/{}", lang, task, n+1, runs);

        let start_time = Utc::now().time();
        let start = sampler.sample_start();

        match cmd.output() {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };

        let energy = sampler.sample_end(start);
        let duration = Utc::now().time() - start_time;
        exports.push(Export { 
            language: lang.to_string(), 
            task: task.to_string(), 
            duration: duration.num_milliseconds(), 
            energy });
    }

    Exports(exports)
}

pub fn summarize(exports: Exports) -> Export {
    let mut summary = Export::default();
    for exp in exports.0 {
        summary.energy += exp.energy;
        summary.duration += exp.duration;
    }

    summary
}
