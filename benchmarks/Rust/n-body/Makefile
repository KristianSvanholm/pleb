compile:
	cargo build --profile=release
	
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./target/release/n-body 50000000" Rust n-body

run:
	./target/release/n-body 50000000

mem:
	/usr/bin/time -v ./target/release/n-body 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./target/release/n-body 50000000
