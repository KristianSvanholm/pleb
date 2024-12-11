compile:	
	cp nbody.fsharpcore Program.fs
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 50000000" FSharp n-body

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 50000000

mem:
	/usr/bin/time -v dotnet ./bin/Release/netcoreapp8.0/tmp.dll 50000000

valgrind:
	valgrind --tool=massif --stacks=yes dotnet ./bin/Release/netcoreapp8.0/tmp.dll 50000000
