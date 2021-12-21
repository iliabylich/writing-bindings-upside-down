CXXFLAGS += -std=c++17

cpp-bindings/print-sizes: cpp-bindings/sizes.cpp cpp-bindings/structs.hpp
	$(CXX) $(CXXFLAGS) cpp-bindings/sizes.cpp -o cpp-bindings/print-sizes
CLEAN += cpp-bindings/print-sizes

cpp-bindings/sizes: cpp-bindings/print-sizes
	rm -f $@
	./cpp-bindings/print-sizes >> $@
CLEAN += cpp-bindings/sizes

cpp-bindings/all.o: cpp-bindings/structs.hpp cpp-bindings/bindings.cpp cpp-bindings/bindings-support.hpp
	$(CXX) $(CXXFLAGS) cpp-bindings/bindings.cpp -g -c -o $@
CLEAN += cpp-bindings/all.o

cpp-bindings/libbindings.a: cpp-bindings/all.o
	$(AR) rc $@ $<
CLEAN += cpp-bindings/libbindings.a
