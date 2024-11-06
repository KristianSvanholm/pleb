use super::metrics::Sampler;

use std::process::Command;

pub fn benchmark(bench: &str) -> u64 {
    let sampler = Sampler::new();

    let start = sampler.get_metrics();

    match Command::new(bench).output() {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{}", e)
    }

    let end = sampler.get_metrics();
    
    end-start
}
