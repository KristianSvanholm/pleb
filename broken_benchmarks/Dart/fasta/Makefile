compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dart  fasta.dart 25000000" Dart fasta

run:
	dart  fasta.dart 25000000

mem:
	/usr/bin/time -v  dart  fasta.dart 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  dart  fasta.dart 25000000
