compile:
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12000" CSharp mandelbrot

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12000

mem:
	/usr/bin/time -v /usr/bin/dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12000

valgrind:
	valgrind --tool=massif --stacks=yes /usr/bin/dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12000
