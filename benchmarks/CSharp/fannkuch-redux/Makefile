compile:
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12" CSharp fannkuch-redux

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12

mem:
	/usr/bin/time -v  dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12

valgrind:
	valgrind --tool=massif --stacks=yes  dotnet ./bin/Release/netcoreapp8.0/tmp.dll 12
