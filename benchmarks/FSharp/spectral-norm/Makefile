compile:	
	cp spectralnorm.fsharpcore-2.fsharpcore Program.fs
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 5500" FSharp spectral-norm

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 5500

mem:
	/usr/bin/time -v  dotnet ./bin/Release/netcoreapp8.0/tmp.dll 5500

valgrind:
	valgrind --tool=massif --stacks=yes  dotnet ./bin/Release/netcoreapp8.0/tmp.dll 5500
