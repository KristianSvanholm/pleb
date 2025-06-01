compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "jruby -Xcompile.fastest=true -Xcompile.invokedynamic=true -J-server -J-Xmn512m -J-Xms2048m -J-Xmx2048m revcomp.rb 0 < ../../revcomp-input25000000.txt" JRuby reverse-complement

run:
	jruby -Xcompile.fastest=true -Xcompile.invokedynamic=true -J-server -J-Xmn512m -J-Xms2048m -J-Xmx2048m revcomp.rb 0 < ../../revcomp-input25000000.txt

mem:
	/usr/bin/time -v jruby -Xcompile.fastest=true -Xcompile.invokedynamic=true -J-server -J-Xmn512m -J-Xms2048m -J-Xmx2048m revcomp.rb 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes jruby -Xcompile.fastest=true -Xcompile.invokedynamic=true -J-server -J-Xmn512m -J-Xms2048m -J-Xmx2048m revcomp.rb 0 < ../../revcomp-input25000000.txt
