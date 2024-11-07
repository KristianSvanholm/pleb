use std::fmt::{Debug, Result};
use std::path::PathBuf;

use powercap::IntelRapl;

#[derive(Debug, Default)]
pub struct Metrics {
  cpu_power: u64, // Joules
}

pub struct Sampler {
  i_rapl: IntelRapl,
}

impl Sampler {
  pub fn new() -> Sampler {
    let i_rapl = match IntelRapl::try_from(PathBuf::from("/sys/class/powercap/intel-rapl")) {
      Ok(ir) => ir,
      Err(e) => panic!("{:?}", e),
    };

    Sampler { i_rapl }
  }

  pub fn get_metrics(&self) -> u64 {
    let res = match self.i_rapl.total_energy() {
      Ok(tot_e) => tot_e,
      Err(e) => {
        panic!("{:?}", e)
      }
    };

    res
  }
}
