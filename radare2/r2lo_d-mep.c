// Copyright (C) 2018 Guillaume Valadon <guillaume@valadon.net>

#include <opcodes/config.h>

#include <r_asm.h>
#include <r_lib.h>

#include <bfd/bfd.h>
#include <include/dis-asm.h>

#include <stdarg.h>

#include "src/lo_utils-mep.h"


static int r2lo_disassemble (RAsm *rasm, RAsmOp *rop, const unsigned char *data, int len) {

    char instruction[R_ASM_BUFSIZE];

    int count = lo_mep_disassemble((char*) &instruction, data, len);
    strcpy(rop->buf_asm, instruction); // GV: check length?

    return count;
}


RAsmPlugin r_asm_plugin_r2lo = {
    .name = "r2lo_mep",
    .arch = "Toshiba MeP",
    .license = "LGPL3",
    .bits = 32|64,
    .desc = "r2lo Toshiba MeP",
    .disassemble = r2lo_disassemble,
    .init = r2lo_init
};


#ifndef CORELIB
struct r_lib_struct_t radare_plugin = {
    .type = R_LIB_TYPE_ASM,
    .data = &r_asm_plugin_r2lo
};
#endif
