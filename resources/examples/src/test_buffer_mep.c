#include <opcodes/config.h>

#include <string.h>
#include <stdint.h>
#include <stdlib.h>

#include <bfd/bfd.h>
#include <include/dis-asm.h>

bfd_byte buffer[] = { 0x53, 0x53, // mov $3,83
	              0x08, 0xd8, 0x01, 0x00,
	              0x53, 0x53,
		      0x30, 0xeb, 0x5b, 0x00
                      }; 
unsigned int buffer_len = 12;


int main( int argc, char ** argv )
{
  disassembler_ftype      disassemble;
  struct disassemble_info info;
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
  init_disassemble_info ( &info, stdout, (fprintf_ftype)fprintf );
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

  asection section;
  section.vma = 0xC00000; // memory offset
  section.flags = 0; // GV: important grep VLIW in dis-mep.C to know why
  info.section = &section;

  info.buffer_length = buffer_len;
  info.buffer        = (bfd_byte*) &buffer;
  info.buffer_vma    = section.vma;

  disassemble_init_for_target ( &info );

  /* Diassemble instructions */
  pc = section.vma;
  for (int i=0; i < 4; i++) {
    printf("Address: 0x%x\n  ", pc);
    count = disassemble(pc, &info);
    pc += count;
    printf("\n====\n");
    fflush(stdout);
  }
  return 0;
}
