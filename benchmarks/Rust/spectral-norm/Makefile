compile:
	cargo build --profile=release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./target/release/spectralnorm 5500" Rust spectral-norm

run:
	./target/release/spectralnorm 5500

mem:
	/usr/bin/time -v ./target/release/spectralnorm 5500

valgrind:
	valgrind --tool=massif --stacks=yes ./target/release/spectralnorm 5500
