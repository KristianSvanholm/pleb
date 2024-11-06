mod macos;
mod linux;

#[cfg(target_os = "macos")]
use macos::metrics::Sampler;
#[cfg(target_os = "linux")]
use linux::metrics::Sampler;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let ms = 100;

  let mut sampler = Sampler::new()?;

  loop {
    let metrics = sampler.get_metrics(ms)?;
    if metrics.cpu_power != 0 {
        println!("{} µj", metrics.cpu_power);
    }
  }

  Ok(())
}
