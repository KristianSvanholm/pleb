compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dart  fannkuchredux.dart-2.dart 12" Dart fannkuch-redux

run:
	dart  fannkuchredux.dart-2.dart 12

mem:
	/usr/bin/time -v dart  fannkuchredux.dart-2.dart 12

valgrind:
	valgrind --tool=massif --stacks=yes dart  fannkuchredux.dart-2.dart 12
