compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dart  mandelbrot.dart 16000" Dart mandelbrot

run:
	dart  mandelbrot.dart 16000

mem:
	/usr/bin/time -v dart  mandelbrot.dart 16000

valgrind:
	valgrind --tool=massif --stacks=yes dart  mandelbrot.dart 16000
