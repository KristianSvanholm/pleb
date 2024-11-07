use core_foundation::dictionary::__CFDictionary;

use super::sources::IOReport;

pub struct Sampler {
    ior: IOReport,
}

impl Sampler {
    pub fn new() -> Sampler {
        let channels = vec![
            ("Energy Model", None), // cpu power
        ];

        let ior = match IOReport::new(channels) {
            Ok(ior) => ior,
            Err(e) => panic!("{:?}", e),
        };

        Sampler { ior }
    }

    pub fn sample_start(&self) -> *const __CFDictionary {
        self.ior.sample()
    }

    pub fn sample_end(&self, prev: *const __CFDictionary) -> f64 {
        self.ior.delta(prev, self.ior.sample())
    }
}
