mod linux;
mod macos;

#[cfg(target_os = "linux")]
use linux::sampler::Sampler;
#[cfg(target_os = "macos")]
use macos::sampler::Sampler;

use chrono::Utc;
use core::fmt;
use std::collections::HashMap;
use serde::Serialize;
use std::fs::{self};
use std::{io, thread, time};
use std::process::Command;

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

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Task {
    path: String,
    pub language: String,
    pub name: String,
}

pub fn list_all(path: String) -> io::Result<Vec<Task>> {
    let mut res: Vec<Task> = vec![];

    // For each language
    for lang in fs::read_dir(&path)? {
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

            // Skip files found
            if !task_path.is_dir() {
                continue;
            }

            if let Some(str) = task.path().to_str() {
                // Get language name and Task
                let mut parts: Vec<&str> = str.split("/").collect();

                // Reverse array
                parts = parts.into_iter().rev().collect();

                res.push(Task {
                    path: str.to_string(),
                    language: parts[1].to_string(),
                    name: parts[0].to_string(),
                });
            }
        }
    }
    Ok(res)
}

pub fn run(tasks: Vec<Task>, runs: u64, cooldown: u64) -> io::Result<Vec<Export>> {
    let mut res: Vec<Export> = vec![];

    let mut counts: HashMap<Task, u64> = HashMap::new();

    let mut first = true; 
    for task in tasks {
        // Cooldown between benchmarks
        if first {
            first = false
        } else {
            println!("Cooling down for {} seconds", &cooldown);
            thread::sleep(time::Duration::from_secs(cooldown));
        }

        // Count runs for each benchmark
        counts.entry(task.clone()).and_modify(|c| *c+=1).or_insert(1);

        // Create make command
        let mut cmd = Command::new("make");
        cmd.arg("-C").arg(&task.path).arg("run");

        // Fetch benchmark count
        let count = match counts.get(&task) {
            Some(c) => c,
            None => &1,
        };
        // Run benchmark
        res.push(benchmark(cmd, runs, &task.language, &task.name, *count));
    }

    res.sort_unstable_by_key(|item| (item.language.to_owned(), item.task.to_owned()));

    Ok(res)
}

pub fn benchmark(mut cmd: Command, runs: u64, lang: &str, task: &str, i: u64) -> Export {
    let sampler = Sampler::new();

    println!("Running {} / {} - {}/{}", lang, task, i, runs);

    let start_time = Utc::now().time();
    let start = sampler.sample_start();

    
    match cmd.output() {
        Ok(_) => (),
        Err(e) => panic!("Encountered error during benchmark: {}", e),
    };

    let energy = sampler.sample_end(start);
    let duration = Utc::now().time() - start_time;

    Export {
        language: lang.to_string(),
        task: task.to_string(),
        duration: duration.num_milliseconds(),
        energy,
    }
}

pub fn compile(tasks: Vec<Task>) {
    for task in tasks {
        // Create make command
        let mut cmd = Command::new("make");
        cmd.arg("-C").arg(task.path).arg("compile");
        let out = match cmd.output() {
            Ok(out) => out,
            Err(e) => {
                println!(
                    "Encountered an error while compiling {} - {}: {}",
                    e, task.language, task.name
                );
                continue;
            }
        };
        let Ok(stderr) = String::from_utf8(out.stderr) else { continue };
        if stderr.len() != 0 {
            println!("stderr:\n {}", stderr);
        }
    }
}
