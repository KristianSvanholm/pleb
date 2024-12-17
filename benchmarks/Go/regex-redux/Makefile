compile:
	go build -o regexredux.go-5.go_run

xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "./regexredux.go-5.go_run 0 < ../../regexredux-input5000000.txt" Go regex-redux

run:
	./regexredux.go-5.go_run 0 < ../../regexredux-input5000000.txt

mem:
	/usr/bin/time -v ./regexredux.go-5.go_run 0 < ../../regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.go-5.go_run 0 < ../../regexredux-input5000000.txt
