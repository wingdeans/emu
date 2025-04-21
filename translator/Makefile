LIB_TRANSLATOR=$(realpath $(CURDIR)/../target/debug/libemu_translator.a)

.PHONY: build test clean

build:
	cargo build --features gbit
	EXTRA_LIB_OBJS=$(LIB_TRANSLATOR) $(MAKE) -C ../cpu/gbit

test: build
	../cpu/gbit/gbit $(GBIT_FLAGS)

clean:
	$(MAKE) -C gbit clean
