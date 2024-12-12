compile:
	cp fasta.typescript fasta.ts
	tsc --alwaysStrict -t ESNEXT  fasta.ts

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node --use_strict fasta.js 25000000" TypeScript fasta

run:
	node --use_strict fasta.js 25000000

clean:
	rm -rf *.js *.ts

mem:
	/usr/bin/time -v node --use_strict fasta.js 25000000

valgrind:
	valgrind --tool=massif --stacks=yes node --use_strict fasta.js 25000000
