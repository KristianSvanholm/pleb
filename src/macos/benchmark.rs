use super::metrics::Sampler;
use std::{thread, time};

use std::process::Command;

pub fn benchmark(bench: &str) -> u64 {
  let sampler = Sampler::new();

  let start = sampler.sample_start();

  //let ms = time::Duration::from_millis(200);
  //thread::sleep(ms);

  match Command::new(bench).output() {
    Ok(res) => println!("{:?}", res),
    Err(e) => println!("{}", e),
  };

  sampler.sample_end(start)
}
