# Thesis
Cross platform programming language energy benchmarking tool.

## Dependencies

- Nix package manager with flakes enabled

then:

`nix develop` to install the rest of the dependencies to a temporary shell.

## Running it

You can run the program either directly via `cargo` through `cargo run --`.  
Otherwise compile it with `cargo build` and run the binary.  

Adding the flag `--help` will provide detailed instructions on how to utilize the programme.

### Note
You need to run `compile` at least once before you can do a `run`.

## MacOS
Utilizes IOReport, an undocumented Apple API.

I performed open source surgery on [vladkens/macmon](https://github.com/vladkens/macmon), to pick out the pieces I needed. Many thanks!

Followed [this](https://medium.com/@vladkens/how-to-get-macos-power-metrics-with-rust-d42b0ad53967) article to achieve this. Again, by [Vladkens](https://github.com/vladkens).

## Linux 
Utilizes Powercap to access Intel RAPL. 

### IMPORTANT

You need to take ownership of the intel RAPL files for the program to work correctly.  
The ownership state of the files reset after each system reboot, so you'll run this command regularily.  
`sudo chown -R $USER /sys/class/powercap/intel-rapl/intel-rapl:0/`

Might add support for AMD RAPL in the future :)

