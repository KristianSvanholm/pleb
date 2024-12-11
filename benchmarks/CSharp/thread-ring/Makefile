compile:
	cp threadring.csharpcore Program.cs
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 50000000"

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 50000000
