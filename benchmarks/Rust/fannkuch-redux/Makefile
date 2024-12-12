compile:
	cargo build --profile=release
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./target/release/fannkuch-redux 12" Rust fannkuch-redux

run:
	./target/release/fannkuch-redux 12

mem:
	/usr/bin/time -v ./target/release/fannkuch-redux 12

valgrind:
	valgrind --tool=massif --stacks=yes ./target/release/fannkuch-redux 12
