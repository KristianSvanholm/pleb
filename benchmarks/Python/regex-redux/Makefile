compile:
	cp regexredux.python3 regexredux.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO regexredux.py 0 < ../../regexredux-input5000000.txt" Python regex-redux

run:
	python -OO regexredux.py 0 < ../../regexredux-input5000000.txt

mem:
	/usr/bin/time -v python -OO regexredux.py 0 < ../../regexredux-input5000000.txt
