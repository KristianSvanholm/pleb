compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dart --old_gen_heap_size=2048 revcomp.dart-3.dart 0 < ../../revcomp-input25000000.txt" Dart reverse-complement

run:
	dart --old_gen_heap_size=2048 revcomp.dart-3.dart 0 < ../../revcomp-input25000000.txt

mem:
	/usr/bin/time -v dart --old_gen_heap_size=2048 revcomp.dart-3.dart 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes dart --old_gen_heap_size=2048 revcomp.dart-3.dart 0 < ../../revcomp-input25000000.txt
