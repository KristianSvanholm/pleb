mod linux;
mod macos;

#[cfg(target_os = "linux")]
use linux::metrics::Sampler;
#[cfg(target_os = "macos")]
use macos::metrics::Sampler;

use std::process::Command;

pub fn benchmark(bench: &str, runs: u64) -> u64 {
    let sampler = Sampler::new();
   
    let mut total = 0;
    for _n in 0..runs {
        let start = sampler.sample_start();

        match Command::new(bench).output() {
            Ok(res) => println!("{:?}", res),
            Err(e) => println!("{}", e),
        };

        total += sampler.sample_end(start);
    };
    total / runs
}
