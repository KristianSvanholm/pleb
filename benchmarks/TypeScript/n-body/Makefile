compile:
	cp nbody.typescript-3.typescript nbody.typescript-3.ts
	tsc --alwaysStrict -t ESNEXT  nbody.typescript-3.ts

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node --use_strict nbody.typescript-3.js 50000000" TypeScript n-body

run:
	node --use_strict nbody.typescript-3.js 50000000

clean:
	rm -rf *.js *.ts

mem:
	/usr/bin/time -v node --use_strict nbody.typescript-3.js 50000000

valgrind:
	valgrind --tool=massif --stacks=yes node --use_strict nbody.typescript-3.js 50000000
