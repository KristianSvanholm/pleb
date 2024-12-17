compile:
	cp pidigits.csharpcore-3.csharpcore Program.cs
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/bin/dotnet ./bin/Release/netcoreapp8.0/tmp.dll 10000" CSharp pidigits

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 10000

mem:
	/usr/bin/time -v  /usr/bin/dotnet ./bin/Release/netcoreapp8.0/tmp.dll 10000

valgrind:
	valgrind --tool=massif --stacks=yes  /usr/bin/dotnet ./bin/Release/netcoreapp8.0/tmp.dll 10000
