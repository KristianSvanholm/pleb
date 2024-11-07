use super::metrics::Sampler;

use std::process::Command;

pub fn benchmark(bench: &str, runs: u64) -> u64 {
    let sampler = Sampler::new();

    let mut total = 0;
    for _n in 0..runs {
        let start = sampler.get_metrics();

        match Command::new(bench).output() {
            Ok(res) => println!("{:?}", res),
            Err(e) => println!("{}", e),
        };

        let end = sampler.get_metrics();

        total += end - start
    };
    total / runs
}
