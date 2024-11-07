mod linux;
mod macos;

#[cfg(target_os = "linux")]
use linux::sampler::Sampler;
#[cfg(target_os = "macos")]
use macos::sampler::Sampler;

use std::process::Command;

pub fn benchmark(mut cmd: Command, runs: u64) -> u64 {
    let sampler = Sampler::new();

    let mut total = 0;
    for _n in 0..runs {
        let start = sampler.sample_start();

        match cmd.output(){
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };

        total += sampler.sample_end(start);
    }
    total / runs
}
