use std::fmt::{Debug, Result};
use std::path::PathBuf;

use powercap::IntelRapl;

#[derive(Debug, Default)]
pub struct Metrics {
    pub cpu_power: u64, // Joules
}

pub struct Sampler {
    i_rapl: IntelRapl,
    prev: u64
}

impl Sampler {
    pub fn new() -> Sampler {
        
        let i_rapl = match IntelRapl::try_from(PathBuf::from("/sys/class/powercap/intel-rapl")) {
            Ok(ir) => ir,
            Err(_e) => panic!("fucked up")
        };

        let prev = i_rapl.total_energy().unwrap();

        Sampler { i_rapl, prev}
    }

    pub fn get_metrics(&self) -> u64{
        let res = match self.i_rapl.total_energy() {
            Ok(tot_e) => tot_e,
            Err(e) => {
                panic!("{:?}", e)
            }
        };
        
        res
    }
}
