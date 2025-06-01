compile:
	javac -d .  mandelbrot.java
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "java   mandelbrot 12000" Java mandelbrot

run:
	java   mandelbrot 12000

mem:
	/usr/bin/time -v java   mandelbrot 12000

valgrind:
	valgrind --tool=massif --stacks=yes java   mandelbrot 12000
