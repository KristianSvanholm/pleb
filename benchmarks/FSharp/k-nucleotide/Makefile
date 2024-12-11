compile:
	cp knucleotide.fsharpcore-4.fsharpcore Program.fs
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 0 < knucleotide-input25000000.txt" FSharp k-nucleotide

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 0 < knucleotide-input25000000.txt

mem:
	/usr/bin/time -v  dotnet ./bin/Release/netcoreapp8.0/tmp.dll 0 < knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  dotnet ./bin/Release/netcoreapp8.0/tmp.dll 0 < knucleotide-input25000000.txt
