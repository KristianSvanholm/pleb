compile:
	cargo build --profile=release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./target/release/pidigits 10000" Rust pidigits

run:
	./target/release/pidigits 10000

mem:
	/usr/bin/time -v ./target/release/pidigits 10000

valgrind:
	valgrind --tool=massif --stacks=yes ./target/release/pidigits 10000
