use powercap::IntelRapl;
use std::path::PathBuf;

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

    pub fn sample_start(&self) -> u64 {
        let res = match self.i_rapl.total_energy() {
            Ok(tot_e) => tot_e,
            Err(e) => {
                panic!("{:?}", e)
            }
        };

        res
    }

    pub fn sample_end(&self, prev: u64) -> u64 {
        self.sample_start().wrapping_sub(prev)
    }
}
