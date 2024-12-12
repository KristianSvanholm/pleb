compile:
	cp nbody.python3 nbody.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO nbody.py 50000000" Python n-body

run:
	python -OO nbody.py 50000000

mem:
	/usr/bin/time -v python -OO nbody.py 50000000
