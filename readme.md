# Thesis

## Macos
Uses IOREPORT

pseudocode:
```rust
// Pre-benchmark steps:
let start = IOReportCreateSamples(...);

// Benchmark
system(benchmark) // wait for this to finish

// Post benchmark steps:
let end = IOReportCreateSamples(...);
let delta = IOReportCreateSampelsDelta(start, end, null());
CFRelease(start as _)
CFRelease(end as _)
let iterator = IOReportIterator::new(delta)

let total_energy = 0;
for x in iterator {
    if x.group == "Energy Model" {
        total_energy += x.item;
    }
}

println!("{}",total_energy);

```

## Linux
Uses intel RAPL through Powercap

```rust
// Pre-benchmark steps:
let start = i_rapl.total_energy()?

// Benchmark
system(benchmark) // wait for this to finish

// Post benchmark steps:
let end = i_rapl.total_energy()?

println!("{}", end-start)

```

