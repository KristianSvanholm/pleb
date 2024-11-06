use std::fmt::Debug;
use std::path::PathBuf;

use powercap::IntelRapl;
use powercap::ReadError;

type WithError<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
pub struct Metrics {
    pub cpu_power: u64, // Joules
}

pub struct Sampler {
    i_rapl: IntelRapl,
    prev: u64
}

impl Sampler {
    pub fn new() -> WithError<Self> {
        
        let i_rapl = match IntelRapl::try_from(PathBuf::from("/sys/class/powercap/intel-rapl")) {
            Ok(ir) => ir,
            Err(_e) => panic!("fucked up ==?=")
        };

        let prev = i_rapl.total_energy().unwrap();

        Ok(Sampler { i_rapl, prev})
    }

    pub fn get_metrics(&mut self, duration: u64) -> WithError<Metrics>{
        let res = match self.i_rapl.total_energy() {
            Ok(tot_e) => tot_e,
            Err(e) => {
                panic!("{:?}", e)
            }
        };
        
        let m = Metrics{ cpu_power: (res-self.prev) };
        self.prev = res;
        Ok(m)

    }
}
