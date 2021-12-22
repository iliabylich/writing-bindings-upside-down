EXTERNAL_BINDINGS = true
BINDINGS_DIR = c-bindings
CARGOFLAGS += --features=external
CFLAGS += -g

c-bindings/print-sizes: c-bindings/sizes.c c-bindings/structs.h
	$(CC) c-bindings/sizes.c $(CFLAGS) -o c-bindings/print-sizes
CLEAN += c-bindings/print-sizes

c-bindings/sizes: c-bindings/print-sizes
	rm -f $@
	./c-bindings/print-sizes > $@
CLEAN += c-bindings/sizes

c-bindings/all.o: c-bindings/structs.h c-bindings/bindings.c c-bindings/bindings-support.h
	$(CC) c-bindings/bindings.c $(CFLAGS) -c -o $@
CLEAN += c-bindings/all.o

c-bindings/libbindings.a: c-bindings/all.o
	$(AR) rc $@ $<
CLEAN += c-bindings/libbindings.a

c-bindings/test-runner: c-bindings/test.c rust-foo/librust-foo-rust.a
	$(CC) c-bindings/test.c $(CFLAGS) -Lrust-foo -lrust-foo-rust -o $@
CLEAN += c-bindings/test-runner

c-bindings/test: c-bindings/test-runner
	./c-bindings/test-runner
