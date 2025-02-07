use powercap::IntelRapl;
use std::path::PathBuf;
use std::fs;

pub struct Sampler {
    i_rapl: IntelRapl,
    max: u64,
}

impl Sampler {
    pub fn new() -> Sampler {
        let path = "/sys/class/powercap/intel-rapl/";

        let i_rapl = match IntelRapl::try_from(PathBuf::from(path)) {
            Ok(ir) => ir,
            Err(e) => panic!("{:?}", e),
        };

        // Ridiculous random 38 bit overflow limit set in max_energy_range_uj file.
        // source: https://arxiv.org/pdf/2401.15985
        let max = fs::read_to_string(path.to_owned() + "intel-rapl:0/max_energy_range_uj")
            .expect("Should have been able to read the file")
            .trim()
            .parse::<u64>()
            .expect("This should have been a number");

        Sampler { i_rapl, max}
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
        let curr = self.sample_start();
        if curr < prev {
            return curr + self.max - prev;
        } else {
            return curr - prev;
        }
    }
}
