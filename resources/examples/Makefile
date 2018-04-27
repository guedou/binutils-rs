# Copyright (C) 2018 Guillaume Valadon <guillaume@valadon.net>

CFLAG_INCLUDES=-Ibinutils-2.29.1/ -Ibinutils-2.29.1/include/ -Ibinutils-2.29.1/bfd/
CFLAGS=-ggdb2
LDFLAGS=-Lbinutils-2.29.1/built/lib -lbfd -lopcodes
ARCHIVES=binutils-2.29.1/libiberty/libiberty.a
TARGETS=test_binary test_buffer_mep test_buffer_x86_64 r2lo_d-mep.so

all: $(TARGETS)

install: r2lo_d-mep.so r2lo_A-mep.so
	cp r2lo_d-mep.so ~/.config/radare2/plugins/
	cp r2lo_A-mep.so ~/.config/radare2/plugins/

clean:
	rm $(TARGETS)

%: src/%.c
	gcc -o $@ $^ $(CFLAG_INCLUDES) $(ARCHIVES) -ldl -lz $(CFLAGS) $(LDFLAGS)
