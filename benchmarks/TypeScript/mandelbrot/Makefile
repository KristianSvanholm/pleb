compile:
	cp mandelbrot.typescript mandelbrot.ts
	tsc --alwaysStrict -t ESNEXT  mandelbrot.ts

xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "node --use_strict mandelbrot.js 16000" TypeScript mandelbrot

run:
	node --use_strict mandelbrot.js 16000

clean:
	rm -rf *.js *.ts
