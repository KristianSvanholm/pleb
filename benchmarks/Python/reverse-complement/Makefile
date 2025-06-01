compile:
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO revcomp.python3-6.py 0 < ../../revcomp-input25000000.txt" Python reverse-complement

run:
	python -OO revcomp.python3-6.py 0 < ../../revcomp-input25000000.txt

mem:
	/usr/bin/time -v python -OO revcomp.python3-6.py 0 < ../../revcomp-input25000000.txt

