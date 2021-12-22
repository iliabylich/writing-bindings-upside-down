EXTERNAL_BINDINGS = true
BINDINGS_DIR = ruby-bindings
CARGOFLAGS += --features=external

ifndef DYLIB
DYLIB = bundle
endif

define compile
ruby ruby-bindings/compile.rb $(1) $(2)
endef

define link_dylib
ruby ruby-bindings/link.rb $(1) $(2)
endef

ruby-bindings/print-sizes: ruby-bindings/sizes.c ruby-bindings/structs.h
	$(call compile,ruby-bindings/sizes.c,ruby-bindings/print-sizes)
CLEAN += ruby-bindings/print-sizes

ruby-bindings/sizes: ruby-bindings/print-sizes
	rm -f $@
	./ruby-bindings/print-sizes > $@
CLEAN += ruby-bindings/sizes

ruby-bindings/bindings.o: ruby-bindings/structs.h ruby-bindings/bindings.c ruby-bindings/bindings-support.h
	EXTRA_CFLAGS="-c" $(call compile,ruby-bindings/bindings.c,$@)
CLEAN += ruby-bindings/bindings.o

ruby-bindings/init.o: ruby-bindings/init.c
	EXTRA_CFLAGS="-c" $(call compile,$<,$@)
CLEAN += ruby-bindings/init.o

ruby-bindings/libbindings.a: ruby-bindings/bindings.o
	$(AR) rc $@ ruby-bindings/bindings.o
CLEAN += ruby-bindings/libbindings.a

ruby-bindings/foo.$(DYLIB): ruby-bindings/init.o rust-foo/librust-foo-rust.a
	$(call link_dylib,ruby-bindings/init.o rust-foo/librust-foo-rust.a,$@)
CLEAN += ruby-bindings/foo.$(DYLIB)

ruby-bindings/test: ruby-bindings/foo.$(DYLIB)
	ruby ruby-bindings/test-runner.rb
