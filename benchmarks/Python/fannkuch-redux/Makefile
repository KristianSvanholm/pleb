compile:
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO fannkuchredux.python3-4.py 12" Python fannkuch-redux

run:
	python -OO fannkuchredux.python3-4.py 12
mem:
	/usr/bin/time -v python -OO fannkuchredux.python3-4.py 12
