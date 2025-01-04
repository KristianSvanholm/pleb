compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "perl mandelbrot.perl 12000" Perl mandelbrot

run:
	perl mandelbrot.perl 12000

mem:
	/usr/bin/time -v perl mandelbrot.perl 12000

valgrind:
	valgrind --tool=massif --stacks=yes perl mandelbrot.perl 12000
