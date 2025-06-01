compile:
	tsc --alwaysStrict -t ESNEXT  revcomp.ts
	
xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "node --use_strict revcomp.js 0 < ../../revcomp-input25000000.txt" TypeScript reverse-complement

run:
	node --use_strict revcomp.js 0 < ../../revcomp-input25000000.txt

clean:
	rm -rf *.js *.ts

mem:
	/usr/bin/time -v node --use_strict revcomp.js 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes node --use_strict revcomp.js 0 < ../../revcomp-input25000000.txt
