libz.a: adler32.o crc32.o deflate.o inffast.o inflate.o inftrees.o trees.o zutil.o
	rm -f $@
	ar qc $@ adler32.o crc32.o deflate.o inffast.o inflate.o inftrees.o trees.o zutil.o

adler32.o: adler32.c
crc32.o: crc32.c
deflate.o: deflate.c
inffast.o: inffast.c
inflate.o: inflate.c
inftrees.o: inftrees.c
trees.o: trees.c
zutil.o: zutil.c

