// Copyright (C) 2018 Guillaume Valadon <guillaume@valadon.net>


#include <r_asm.h>
#include <r_lib.h>

#include "src/lo_utils-mep.h"


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
    r2lo_init (NULL); // GV: set addr !
    char instruction[R_ASM_BUFSIZE];

    int count = lo_mep_disassemble((char*) &instruction, data, len);
    rop->size = count;

    char *opcode = strtok(&instruction, " ");
    if (strcmp("JMP", opcode) == 0) { // GV: instr.dstflow
      rop->eob = 1;

      opcode = strtok(NULL, " ");
      if (opcode[0] == '0' && opcode[1] == 'x') {
        unsigned long int addr = strtoul(opcode, NULL, 16);  // GV: check errors ...
        rop->type = R_ANAL_OP_TYPE_JMP;
        rop->jump = addr;
      } else {
        rop->type = R_ANAL_OP_TYPE_UJMP;
      }
    }
    else if (strcmp("RET", opcode) == 0) {
      rop-> type = R_ANAL_OP_TYPE_RET;
    }

    return rop->size;
}


struct r_anal_plugin_t r_anal_plugin_r2m2 = {
    .name = "r2lo_mep",
    .arch = "Toshiba MeP",
    .license = "LGPL3",
    .bits = 32|64,
    .desc = "r2lo Toshiba MeP",
    .op = analyze,
};

#ifndef CORELIB
struct r_lib_struct_t radare_plugin = {
    .type = R_LIB_TYPE_ANAL,
    .data = &r_anal_plugin_r2m2
};
#endif
