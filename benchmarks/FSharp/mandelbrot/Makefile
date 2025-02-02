compile:
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12000" FSharp mandelbrot

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12000

mem:
	/usr/bin/time -v  dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12000

valgrind:
	valgrind --tool=massif --stacks=yes  dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12000
