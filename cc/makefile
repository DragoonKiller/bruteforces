run_all_tests : rebuild
	cd tests && cd test1 && ..\..\target\debug\cc.exe -i templates -o submit.cpp x.cpp -h -g
	cd tests && cd test2 && ..\..\target\debug\cc.exe -o ./submits/submit.cpp x.cpp -h
	cd tests && cd test3 && ..\..\target\debug\cc.exe -i templates -o submit.cpp x.cpp -g
	cd tests && cd test4 && ..\..\target\debug\cc.exe -i templates -o submit.cpp x.cpp

build_and_run : rebuild run
	
run :
	.\target\debug\cc.exe -i templates -o submit.cpp x.cpp

rebuild :
	@cargo build