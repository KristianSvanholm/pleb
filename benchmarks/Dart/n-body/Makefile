compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dart  nbody.dart-3.dart 50000000" Dart n-body

run:
	dart  nbody.dart-3.dart 50000000

mem:
	/usr/bin/time -v dart  nbody.dart-3.dart 50000000

valgrind:
	valgrind --tool=massif --stacks=yes dart  nbody.dart-3.dart 50000000
