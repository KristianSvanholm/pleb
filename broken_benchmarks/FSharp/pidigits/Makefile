compile:	
	cp pidigits.fsharpcore-3.fsharpcore Program.fs
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 10000" FSharp pidigits

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 10000

mem:
	/usr/bin/time -v dotnet ./bin/Release/netcoreapp8.0/tmp.dll 10000

valgrind:
	valgrind --tool=massif --stacks=yes .dotnet ./bin/Release/netcoreapp8.0/tmp.dll 10000
