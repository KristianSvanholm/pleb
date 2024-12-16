compile:
	cp regexredux.jruby-2.jruby regexredux.rb

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "jruby -Xcompile.fastest=true -Xcompile.invokedynamic=true -J-server -J-Xmn512m -J-Xms2048m -J-Xmx2048m regexredux.rb 0 < ../../regexredux-input5000000.txt" JRuby regex-redux

run:
	jruby -Xcompile.fastest=true -Xcompile.invokedynamic=true -J-server -J-Xmn512m -J-Xms2048m -J-Xmx2048m regexredux.rb 0 < ../../regexredux-input5000000.txt

mem:
	/usr/bin/time -v  jruby -Xcompile.fastest=true -Xcompile.invokedynamic=true -J-server -J-Xmn512m -J-Xms2048m -J-Xmx2048m regexredux.rb 0 < ../../regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  jruby -Xcompile.fastest=true -Xcompile.invokedynamic=true -J-server -J-Xmn512m -J-Xms2048m -J-Xmx2048m regexredux.rb 0 < ../../regexredux-input5000000.txt
