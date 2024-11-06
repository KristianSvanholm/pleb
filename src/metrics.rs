use crate::sources::{cfio_watts, IOReport};

type WithError<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
pub struct Metrics {
  pub cpu_power: f32, // Watts
}

pub struct Sampler {
  ior: IOReport,
}

impl Sampler {
  pub fn new() -> WithError<Self> {
    let channels = vec![
      ("Energy Model", None), // cpu power
    ];

    let ior = IOReport::new(channels)?;

    Ok(Sampler { ior })
  }

  pub fn get_metrics(&mut self, duration: u64) -> WithError<Metrics> {
    let mut rs = Metrics::default();

    for x in self.ior.get_sample(duration) {
      if x.group == "Energy Model" {
        rs.cpu_power += cfio_watts(x.item, &x.unit, duration)?
      }
    }

    Ok(rs)
  }
}
