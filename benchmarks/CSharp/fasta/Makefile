compile:
	cp fasta.csharpcore Program.cs
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0.11/tmp.dll 25000000" CSharp fasta

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 25000000

mem:
	/usr/bin/time -v  /usr/bin/dotnet ./bin/Release/netcoreapp8.0/tmp.dll 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  /usr/bin/dotnet ./bin/Release/netcoreapp8.0/tmp.dll 25000000
