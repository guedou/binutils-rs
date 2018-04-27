// Copyright (C) 2018 Guillaume Valadon <guillaume@valadon.net>
//
#include <opcodes/config.h>

#include <r_asm.h>
#include <r_lib.h>

#include <bfd/bfd.h>
#include <include/dis-asm.h>

#include <stdarg.h>


static disassembler_ftype disassemble;
static struct disassemble_info info;
static asection section;


char tmp_buf_asm[R_ASM_BUFSIZE];
char *tmp_buf_asm_ptr = tmp_buf_asm;
void copy_buffer(void * useless, const char* format, ...) {

    va_list ap;
    int i;

    va_start(ap, format);
    vsnprintf(tmp_buf_asm_ptr, R_ASM_BUFSIZE-(tmp_buf_asm_ptr-tmp_buf_asm), format, ap);
    tmp_buf_asm_ptr = tmp_buf_asm + strlen(tmp_buf_asm);
    va_end(ap);
}


bool r2lo_init(void *user) {
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


int lo_mep_disassemble(char* buffer, char* data, int len) {

    unsigned long count = 0;

    info.buffer_length = len;
    info.buffer_vma    = section.vma;
    info.buffer        = (bfd_byte*) data;

    bzero(tmp_buf_asm, R_ASM_BUFSIZE);
    tmp_buf_asm_ptr = tmp_buf_asm;
    count = disassemble(0, &info);

    //strcpy(buffer, tmp_buf_asm); // GV: check length?
    
    memset(buffer, 0, R_ASM_BUFSIZE);
    int offset = 0;
    for(int i=0; i < strlen(tmp_buf_asm); i++) {
      if (tmp_buf_asm[i] == '$')
      {
        if (isdigit(tmp_buf_asm[i+1])) { // GV: offset error
          buffer[i-offset] = 'R';
        } else {
          offset += 1;
        }
        continue;
      }
      if (tmp_buf_asm[i] != 'x') {
        buffer[i-offset] = toupper(tmp_buf_asm[i]);
      } else {
        buffer[i-offset] = tmp_buf_asm[i];
      }
      if (tmp_buf_asm[i] == ',')
      {
        offset -= 1;
        buffer[i-offset] = ' ';
      }
    }

    return count;
}
