// Copyright (C) 2018 Guillaume Valadon <guillaume@valadon.net>

#include <opcodes/config.h>

#include <r_asm.h>
#include <r_lib.h>

#include <bfd/bfd.h>
#include <include/dis-asm.h>

#include <stdarg.h>

char xxx[1024]; // GV: must be the same size as rop->buf_asm
char *ptr = xxx;
void copy_buffer(void * useless, const char* format, ...) {

    va_list ap;
    int i;

    va_start(ap, format);
    vsnprintf(ptr, 1024-(ptr-xxx), format, ap);
    ptr = xxx + strlen(xxx);
    va_end(ap);
}

static disassembler_ftype      disassemble;
static struct disassemble_info info;
static asection section;

static int r2lo_disassemble (RAsm *rasm, RAsmOp *rop, const unsigned char *data, int len) {

    unsigned long count = 0;

    info.buffer_length = len;
    info.buffer_vma    = section.vma;
    info.buffer        = (bfd_byte*) data;

    bzero(xxx, 1024);
    ptr = xxx;
    count = disassemble(0, &info);
    rop->size = count;

    //strcpy(rop->buf_asm, xxx);
    memset(rop->buf_asm, 0, 64); // GV: use the correct constant
    int offset = 0;
    for(int i=0; i < strlen(xxx); i++) {
      if (xxx[i] == '$')
      {
        if (isdigit(xxx[i+1])) { // GV: offset error
          rop->buf_asm[i-offset] = 'R';
        } else {
          offset += 1;
        }
        continue;
      }
      if (xxx[i] != 'x') {
        rop->buf_asm[i-offset] = toupper(xxx[i]);
      } else {
        rop->buf_asm[i-offset] = xxx[i];
      }
      if (xxx[i] == ',')
      {
        offset -= 1;
        rop->buf_asm[i-offset] = ' ';
      }
    }

    return rop->size;
}


static bool init(void *user) {
  // Load the libpython2.7 dynamic library
  unsigned long count, pc;

  /* Construct a specific disassembler */
  enum bfd_architecture arch = bfd_arch_mep;
  bfd_boolean big_endian = BFD_ENDIAN_BIG;
  unsigned long mach = bfd_mach_mep;
  disassemble = disassembler (arch, big_endian, mach, NULL);
  if ( disassemble == NULL )
  {
    printf( "Error creating disassembler\n" );
    return -1;
  }

  /* Construct and configure the disassembler_info structure */
  //init_disassemble_info ( &info, stdout, (fprintf_ftype)fprintf );
  init_disassemble_info ( &info, stdout, (fprintf_ftype)copy_buffer );
  info.arch = arch;
  info.mach = mach;
  info.read_memory_func = buffer_read_memory;

  // From GDB
  info.octets_per_byte = 1;
  info.skip_zeroes = 256;
  info.skip_zeroes_at_end = 0;
  info.insn_type = dis_noninsn;
  info.target = 0;
  info.target2 = 0;
  info.stop_vma = 0;
  info.flags = 1610612736;

  //section.vma = 0xC00000; // memory offset
  section.vma = 0; // memory offset
  section.flags = 0; // GV: important grep VLIW in dis-mep.C to know why
  info.section = &section;

  disassemble_init_for_target ( &info );

  return true;
}


RAsmPlugin r_asm_plugin_r2lo = {
    .name = "r2lo_mep",
    .arch = "Toshiba MeP",
    .license = "LGPL3",
    .bits = 32|64,
    .desc = "r2lo Toshiba MeP",
    .disassemble = r2lo_disassemble,
    .init = init
};


#ifndef CORELIB
struct r_lib_struct_t radare_plugin = {
    .type = R_LIB_TYPE_ASM,
    .data = &r_asm_plugin_r2lo
};
#endif
