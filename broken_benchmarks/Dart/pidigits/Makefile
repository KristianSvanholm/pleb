compile:

xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "dart  pidigits.dart 2000" Dart pidigits

run:
	dart  pidigits.dart 2000

mem:
	/usr/bin/time -v  dart  pidigits.dart 2000

valgrind:
	valgrind --tool=massif --stacks=yes  dart  pidigits.dart 2000
