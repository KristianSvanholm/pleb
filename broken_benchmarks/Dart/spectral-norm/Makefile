compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dart  spectralnorm.dart 5500" Dart spectral-norm

run:
	dart  spectralnorm.dart 5500

mem:
	/usr/bin/time -v dart  spectralnorm.dart 5500

valgrind:
	valgrind --tool=massif --stacks=yes dart  spectralnorm.dart 5500
