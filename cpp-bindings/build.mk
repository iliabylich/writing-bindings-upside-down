EXTERNAL_BINDINGS = true
BINDINGS_DIR = cpp-bindings
CARGOFLAGS += --features=external,link-with-cxx-runtime
CXXFLAGS += -std=c++17 -g -lpthread -ldl -lm -fPIE

cpp-bindings/print-sizes: cpp-bindings/sizes.cpp cpp-bindings/structs.hpp
	$(CXX) $(CXXFLAGS) cpp-bindings/sizes.cpp -o cpp-bindings/print-sizes
CLEAN += cpp-bindings/print-sizes

cpp-bindings/sizes: cpp-bindings/print-sizes
	rm -f $@
	./cpp-bindings/print-sizes > $@
CLEAN += cpp-bindings/sizes

cpp-bindings/bindings.o: cpp-bindings/structs.hpp cpp-bindings/bindings.cpp cpp-bindings/bindings-support.hpp
	$(CXX) $(CXXFLAGS) cpp-bindings/bindings.cpp -c -o $@
CLEAN += cpp-bindings/bindings.o

cpp-bindings/bindings-support.o: cpp-bindings/structs.hpp cpp-bindings/bindings-support.cpp cpp-bindings/bindings-support.hpp
	$(CXX) $(CXXFLAGS) cpp-bindings/bindings-support.cpp -c -o $@
CLEAN += cpp-bindings/bindings-support.o

cpp-bindings/libbindings.a: cpp-bindings/bindings.o cpp-bindings/bindings-support.o
	$(AR) rc $@ cpp-bindings/bindings.o cpp-bindings/bindings-support.o
CLEAN += cpp-bindings/libbindings.a

cpp-bindings/test-runner: cpp-bindings/test.cpp rust-foo/librust-foo-rust.a
	$(CXX) cpp-bindings/test.cpp $(CXXFLAGS) -Lrust-foo -lrust-foo-rust -o $@
CLEAN += cpp-bindings/test-runner

cpp-bindings/test: cpp-bindings/test-runner
	./cpp-bindings/test-runner
