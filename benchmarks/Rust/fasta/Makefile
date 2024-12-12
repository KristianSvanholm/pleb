compile:
	cargo build --profile=release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./target/release/fasta 25000000" Rust fasta

run:
	./target/release/fasta 25000000

mem:
	/usr/bin/time -v ./target/release/fasta 25000000

valgrind:
	valgrind --tool=massif --stacks=yes ./target/release/fasta 25000000
