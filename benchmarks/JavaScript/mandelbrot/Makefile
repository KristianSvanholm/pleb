compile:
	cp -L mandelbrot.node mandelbrot.js
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node mandelbrot.js 12000" JavaScript mandelbrot

run:
	node mandelbrot.js 12000

mem:
	/usr/bin/time -v node mandelbrot.js 12000

valgrind:
	valgrind --tool=massif --stacks=yes node mandelbrot.js 12000
