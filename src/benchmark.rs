mod linux;
mod macos;

#[cfg(target_os = "linux")]
use linux::sampler::Sampler;
#[cfg(target_os = "macos")]
use macos::sampler::Sampler;

use chrono::{Duration, Utc};
use core::fmt;
use std::process::Command;

#[derive(Debug, Default)]
pub struct Export {
    duration: Duration,
    energy: f64,
}

impl fmt::Display for Export {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} µj, {} ms, {:.5} w",
            self.energy,
            self.duration.num_milliseconds(),
            self.energy / self.duration.num_microseconds().expect("bull") as f64
        )
    }
}

pub struct Exports(Vec<Export>);

impl fmt::Display for Exports {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Results:\n")?;
        for (i, v) in self.0.iter().enumerate() {
            write!(
                f,
                "{:>2} - {:>10} µj, {:>3} ms, {:>10.5} w\n",
                i,
                v.energy,
                v.duration.num_milliseconds(),
                v.energy as f32 / v.duration.num_microseconds().expect("bull") as f32
            )?;
        }
        Ok(())
    }
}

pub fn benchmark(mut cmd: Command, runs: u64) -> Exports {
    let sampler = Sampler::new();

    let mut exports: Vec<Export> = vec![];

    for _n in 0..runs {
        let start_time = Utc::now().time();
        let start = sampler.sample_start();

        match cmd.output() {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };

        let energy = sampler.sample_end(start);
        let duration = Utc::now().time() - start_time;
        exports.push(Export { duration, energy });
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
