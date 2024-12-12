compile:
	cp fannkuchredux.typescript fannkuchredux.ts
	tsc --alwaysStrict -t ESNEXT  fannkuchredux.ts

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node --use_strict fannkuchredux.js 12" TypeScript fannkuch-redux

run:
	node --use_strict fannkuchredux.js 12

clean:
	rm -rf *.js *.ts

mem:
	/usr/bin/time -v node --use_strict fannkuchredux.js 12

valgrind:
	valgrind --tool=massif --stacks=yes node --use_strict fannkuchredux.js 12
