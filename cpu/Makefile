LIB_CPU=$(realpath $(CURDIR)/../target/debug/libcpu.a)
GBIT_FLAGS=$(if $(BYPASS_ERROR),-k,)

.PHONY: test

test:
	cargo build --features api
	EXTRA_LIB_OBJS=$(LIB_CPU) $(MAKE) -C gbit
	./gbit/gbit $(GBIT_FLAGS)

clean:
	$(MAKE) -C gbit clean
