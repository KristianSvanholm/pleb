compile:
	cp binarytrees.python3 binarytrees.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO binarytrees.py 21" Python binary-trees

run:
	python -OO binarytrees.py 21

mem:
	/usr/bin/time -v python -OO binarytrees.py 21
