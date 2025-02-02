compile:
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO pidigits.python3-2.py 10000" Python pidigits

run:
	python -OO pidigits.python3-2.py 10000
mem:
	/usr/bin/time -v python -OO pidigits.python3-2.py 10000
