c-bindings/print-sizes: c-bindings/sizes.c c-bindings/structs.h
	$(CC) c-bindings/sizes.c -o c-bindings/print-sizes

c-bindings/sizes: c-bindings/print-sizes
	rm -f $@
	./c-bindings/print-sizes >> $@
