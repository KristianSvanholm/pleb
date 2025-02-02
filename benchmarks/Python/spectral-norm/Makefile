compile:
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO spectralnorm.python3-5.py 5500" Python spectral-norm

run:
	python -OO spectralnorm.python3-5.py 5500

mem:
	/usr/bin/time -v python -OO spectralnorm.python3-5.py 5500
