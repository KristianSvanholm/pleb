compile:
	cargo build --profile=release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./target/release/mandelbrot 12000" Rust mandelbrot

run:
	./target/release/mandelbrot 12000

mem:
	/usr/bin/time -v  ./target/release/mandelbrot 12000

valgrind:
	valgrind --tool=massif --stacks=yes  ./target/release/mandelbrot 12000
