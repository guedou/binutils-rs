# r2lo test
R2_PLUGIN_PATH=$(HOME)/.config/radare2/plugins/
R2_INCLUDES_PATH=$(shell r2 -hh|grep INCDIR|awk '{print $$2}')
R2_CFLAGS=-g -fPIC $(shell pkg-config --cflags r_asm) -shared
r2lo_d-mep.so: src/lo_utils-mep.c r2lo_d-mep.c 
	$(CC) $(R2_CFLAGS) -I$(R2_INCLUDES_PATH) $^ -o $@ $(LINKER_OPTIONS) $(CFLAG_INCLUDES) $(ARCHIVES) $(LDFLAGS)
r2lo_A-mep.so: src/lo_utils-mep.c r2lo_A-mep.c 
	$(CC) $(R2_CFLAGS) -I$(R2_INCLUDES_PATH) $^ -o $@ $(LINKER_OPTIONS) $(CFLAG_INCLUDES) $(ARCHIVES) $(LDFLAGS)
