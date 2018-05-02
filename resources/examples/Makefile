# Copyright (C) 2018 Guillaume Valadon <guillaume@valadon.net>

BINUTILS_BUILT_DIR=../../built/

CFLAG_INCLUDES=-I$(BINUTILS_BUILT_DIR)/include/
CFLAGS=-ggdb2
LDFLAGS=-L$(BINUTILS_BUILT_DIR)/lib/ -lbfd -lopcodes -Wl,-rpath,$(BINUTILS_BUILT_DIR)/lib/
TARGETS=test_binary test_buffer_mep test_buffer_x86_64

all: $(TARGETS)

.IGNORE: clean
clean:
	@rm $(TARGETS)

%: src/%.c
	gcc -o $@ $^ $(CFLAG_INCLUDES) -ldl -lz $(CFLAGS) $(LDFLAGS)
