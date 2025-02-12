LIB_CPU=$(realpath $(CURDIR)/../build/debug/libcpu.a)
GBIT_FLAGS=$(if $(BYPASS_ERROR),-k,)

.PHONY: test

test:
	cargo build --features api
	EXTRA_BIN_OBJS=$(LIB_CPU) $(MAKE) -C gbit
	./gbit/gbit $(GBIT_FLAGS)

clean:
	$(MAKE) -C gbit
