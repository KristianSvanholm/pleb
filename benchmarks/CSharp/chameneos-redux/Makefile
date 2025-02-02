compile:
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 6000000"

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 6000000

mem:
	/usr/bin/time -v dotnet ./bin/Release/netcoreapp8.0/tmp.dll 6000000

valgrind:
	valgrind --tool=massif --stacks=yes dotnet ./bin/Release/netcoreapp8.0/tmp.dll 6000000
