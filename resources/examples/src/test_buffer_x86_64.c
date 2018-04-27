#include <opcodes/config.h>

#include <string.h>
#include <stdint.h>

#include <bfd/bfd.h>
#include <include/dis-asm.h>

bfd_byte buffer[] = { 0xc3, 0x90, 0x66, 0x90 };
int buffer_len = 4;
int buffer_ptr = 0;

int pouet(bfd_vma memaddr, bfd_byte *myaddr, unsigned int length, struct disassemble_info *dinfo) {
  if (length > buffer_len)
    return 1;

  memcpy (myaddr, &buffer[buffer_ptr], length);
  buffer_ptr += length;

  return 0;
}

int main( int argc, char ** argv )
{
  disassembler_ftype      disassemble;
  struct disassemble_info info;
  unsigned long count, pc;

  /* Construct a specific disassembler */
  enum bfd_architecture arch = bfd_arch_i386;
  bfd_boolean big_endian = FALSE;
  unsigned long mach = bfd_mach_x86_64;
  disassemble = disassembler (arch, big_endian, mach, NULL);
  if ( disassemble == NULL )
  {
    printf( "Error creating disassembler\n" );
    return -1;
  }

  /* Construct and configure the disassembler_info structure */
  init_disassemble_info ( &info, stdout, (fprintf_ftype)fprintf );
  info.arch = arch;
  info.mach = mach;
  info.read_memory_func = pouet;

  disassemble_init_for_target ( &info );

  /* Diassemble a single instruction */
  pc = 0;
  for (int i=0; i < 3; i++) {
    printf("Address: 0x%x\n", pc );
    count = disassemble( pc, &info );
    pc += count;
    printf("\nType: 0x%x\n", info.insn_type );
    printf("====\n");
  }
  return 0;
}
