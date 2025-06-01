compile:
	g++ -c  -O3 -fomit-frame-pointer -std=c++17 fasta.gpp-9.c++ -o fasta.gpp-9.c++.o &&  \
        g++ fasta.gpp-9.c++.o -o fasta.gpp-9.gpp_run -lpthread 


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./fasta.gpp-9.gpp_run 25000000" C++ fasta

run:
	./fasta.gpp-9.gpp_run 25000000

mem:
	/usr/bin/time -v ./fasta.gpp-9.gpp_run 25000000

valgrind:
	valgrind --tool=massif --stacks=yes ./fasta.gpp-9.gpp_run 25000000

