compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dart  binarytrees.dart 21" Dart binary-trees

run:
	dart  binarytrees.dart 21

mem:
	/usr/bin/time -v  dart  binarytrees.dart 21

valgrind:
	valgrind --tool=massif --stacks=yes  dart  binarytrees.dart 21
