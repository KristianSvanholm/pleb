compile:
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO fasta.python3-3.py 25000000" Python fasta

run:
	python -OO fasta.python3-3.py 25000000

make mem:
	/usr/bin/time -v  python -OO fasta.python3-3.py 25000000
