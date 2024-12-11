compile:
	cp regexredux.csharpcore-4.csharpcore Program.cs
	dotnet build -c Release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "dotnet ./bin/Release/netcoreapp8.0/tmp.dll 0 < ../../../../regexredux-input5000000.txt" CSharp regex-redux

run:
	dotnet ./bin/Release/netcoreapp8.0/tmp.dll 0 < ../../../../regexredux-input5000000.txt

mem:
	/usr/bin/time -v dotnet ./bin/Release/netcoreapp8.0/tmp.dll 0 < ../../../../regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes dotnet ./bin/Release/netcoreapp8.0/tmp.dll 0 < ../../../../regexredux-input5000000.txt
