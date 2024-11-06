mod macos;
mod linux;

#[cfg(target_os = "linux")]
use linux::benchmark::benchmark;
#[cfg(target_os = "macos")]
use macos::benchmark::benchmark;

fn main() {
    println!("{} µj", benchmark("pwd"));
}
