mod linux;
mod macos;

#[cfg(target_os = "linux")]
use linux::sampler::Sampler;
#[cfg(target_os = "macos")]
use macos::sampler::Sampler;

use chrono::Utc;
use core::fmt;
use serde::Serialize;
use std::fs::{self};
use std::io;

use std::process::Command;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Export {
    pub language: String,
    pub task: String,
    pub duration: i64,
    pub energy: f64,
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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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

                if parts[0].to_string() == "node_modules" {
                    continue;
                }

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

pub fn run(task: Task) -> Export {
    // Create make command
    let mut cmd = Command::new("make");
    cmd.arg("-C").arg(&task.path).arg("run");
    // Run benchmark
    benchmark(cmd, task.language, task.name)
}

pub fn benchmark(mut cmd: Command, lang: String, task: String) -> Export {
    let sampler = Sampler::new();

    let start_time = Utc::now().time();
    let start = sampler.sample_start();

    match cmd.output() {
        Ok(_) => (),
        Err(e) => panic!("Encountered error during benchmark: {}", e),
    };

    let energy = sampler.sample_end(start) as f64;
    let duration = Utc::now().time() - start_time;

    Export {
        language: lang.to_string(),
        task: task.to_string(),
        duration: duration.num_milliseconds(),
        energy,
    }
}

// Todo:: Rework
pub fn compile(task: &Task) {
    // Create make command
    let mut cmd = Command::new("make");
    cmd.arg("-C").arg(task.path.clone()).arg("compile");
    let out = match cmd.output() {
        Ok(out) => out,
        Err(e) => {
            println!(
                "Encountered an error while compiling {} - {}:\n {}",
                task.language, task.name, e
            );
            return;
        }
    };
    let Ok(stderr) = String::from_utf8(out.stderr) else { return };
    if stderr.len() != 0 {
        println!(
            "Encountered an error while compiling {} - {}:\n {}",
            task.language, task.name, stderr
        );
    }
}
