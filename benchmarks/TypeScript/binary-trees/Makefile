compile:
	tsc --alwaysStrict -t ESNEXT  binarytrees.typescript-2.ts

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "node --use_strict binarytrees.typescript-2.js 21" TypeScript binary-trees

run:
	node --use_strict binarytrees.typescript-2.js 21

clean:
	rm -rf *.js *.ts

mem:
	/usr/bin/time -v node --use_strict binarytrees.typescript-2.js 21

valgrind:
	valgrind --tool=massif --stacks=yes node --use_strict binarytrees.typescript-2.js 21

