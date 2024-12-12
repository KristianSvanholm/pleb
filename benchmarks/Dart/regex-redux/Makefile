compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dart  regexredux.dart-2.dart 0 < ../../regexredux-input5000000.txt" Dart regex-redux

run:
	dart  regexredux.dart-2.dart 0 < ../../regexredux-input5000000.txt

mem:
	/usr/bin/time -v  dart  regexredux.dart-2.dart 0 < ../../regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  dart  regexredux.dart-2.dart 0 < ../../regexredux-input5000000.txt
