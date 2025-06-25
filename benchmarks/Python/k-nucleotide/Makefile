compile:
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO knucleotide.python3-3.py 0 < ../../knucleotide-input25000000.txt" Python k-nucleotide

run:
	python -OO knucleotide.python3-3.py 0 < ../../knucleotide-input25000000.txt

mem:
	/usr/bin/time -v python -OO knucleotide.python3-3.py 0 < ../../knucleotide-input25000000.txt
