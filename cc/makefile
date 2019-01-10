run_all_tests : rebuild
	cd tests && cd test1 && ..\..\target\debug\cc.exe -i templates -o submit.cpp x.cpp -u
	cd tests && cd test2 && ..\..\target\debug\cc.exe -o ./submits/submit.cpp x.cpp -u
	cd tests && cd test3 && ..\..\target\debug\cc.exe -i templates -o submit.cpp x.cpp
	cd tests && cd test4 && ..\..\target\debug\cc.exe -i templates -o submit.cpp x.cpp -m

build_and_run : rebuild run
	
run :
	.\target\debug\cc.exe -i templates -o submit.cpp x.cpp

rebuild :
	@cargo build