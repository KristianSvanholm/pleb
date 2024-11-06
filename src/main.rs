mod debug;
mod metrics;
mod sources;

use metrics::Sampler;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let msec = 100;

  let mut sampler = Sampler::new()?;

  loop {
    let metrics = sampler.get_metrics(msec)?;
    println!("{:?}", metrics);
  }

  Ok(())
}
