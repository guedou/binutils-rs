// Copyright (C) 2016 Guillaume Valadon <guillaume@valadon.net>

// r2m2 plugin that uses miasm2 as a radare2 analysis and emulation backend

#include <r_asm.h>
#include <r_lib.h>
#include "r2m2.h"
#include "r2m2_Ae.h"


static int analyze (RAnal *unused, RAnalOp *rop, ut64 addr, const ut8 *data, int len) {
     // If the size is set, the instruction was already processed
     // Note: this is a trick to enhance performances, as radare2 calls analyze()
     //       several times.
     if (rop->size) {
         return rop->size;
     }

    // Analyze an instruction using miasm
    memset (rop, 0, sizeof (RAnalOp));
    rop->type = R_ANAL_OP_TYPE_UNK;

    // GV: must implement a generic disassembler init and function

    return rop->size;
}


struct r_anal_plugin_t r_anal_plugin_r2m2 = {
    .name = "r2m2",
    .arch = "r2m2",
    .license = "LGPL3",
    .bits = R2M2_ARCH_BITS, // GV: seems fishy
    .desc = "miasm2 backend",
    .op = analyze,
};

#ifndef CORELIB
struct r_lib_struct_t radare_plugin = {
    .type = R_LIB_TYPE_ANAL,
    .data = &r_anal_plugin_r2m2
};
#endif
