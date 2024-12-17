compile:
	cargo build --profile=release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./target/release/k-nucleotide 0 < ../../knucleotide-input25000000.txt" Rust k-nucleotide

run:
	./target/release/k-nucleotide 0 < ../../knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./target/release/k-nucleotide 0 < ../../knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./target/release/k-nucleotide 0 < ../../knucleotide-input25000000.txt
