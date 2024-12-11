compile:
	cp fasta.fsharpcore Program.fs
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 25000000" FSharp fasta

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 25000000

mem:
	/usr/bin/time -v dotnet ./bin/Release/netcoreapp8.0/tmp.dll 25000000

valgrind:
	valgrind --tool=massif --stacks=yes dotnet ./bin/Release/netcoreapp8.0/tmp.dll 25000000
