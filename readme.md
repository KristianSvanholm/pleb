# Thesis
Cross platform programming language energy benchmarking tool.

## Dependencies

- Nix package manager with flakes enabled

then:

`nix develop` to install the rest of the dependencies to a temporary shell.

## Running it

On macos, simply `cargo run`

On Linux, you need sudo. `cargo build`, then navigate to `target/debug/` and run the program with `sudo ./thesis`

Optional flag `-r` sets count of revisions per task.

## MacOS
Utilizes IOReport, an undocumented Apple API.

I performed open source surgery on [vladkens/macmon](https://github.com/vladkens/macmon), to pick out the pieces I needed. Many thanks!

Followed [this](https://medium.com/@vladkens/how-to-get-macos-power-metrics-with-rust-d42b0ad53967) article to achieve this. Again, by [Vladkens](https://github.com/vladkens).

## Linux 
Utilizes Powercap to access Intel RAPL. 

Might add support for AMD RAPL in the future :)

