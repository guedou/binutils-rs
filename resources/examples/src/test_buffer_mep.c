// Guillaume Valadon <guillaume@valadon.net>

#include <config.h>

#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include <bfd.h>
#include <dis-asm.h>

bfd_byte buffer[] = { 0x53, 0x53,
	              0x08, 0xd8, 0x01, 0x00,
	              0x53, 0x53,
		      0x30, 0xeb, 0x5b, 0x00
                    }; 
unsigned int buffer_len = 12;


int main (int argc, char ** argv) {
  bfd                     *bfdFile;
  asection                *section;
  disassembler_ftype      disassemble;
  struct disassemble_info info;
  unsigned long           count, pc;

  /* Open /dev/null as a binary target with libbfd */
  bfdFile = bfd_openr ("/dev/null", "binary");
  if (bfdFile == NULL) {
    printf ("Error [%x]: %s\n", bfd_get_error (),
                                bfd_errmsg (bfd_get_error ()));
    return EXIT_FAILURE;
  }

  /* Check the file format */
  if (!bfd_check_format (bfdFile, bfd_object)) {
    printf ("Error [%x]: %s\n", bfd_get_error (),
                                bfd_errmsg (bfd_get_error ()));
    return EXIT_FAILURE;
  }

  /* Retrieve the ELF .data code section */
  section = bfd_get_section_by_name (bfdFile, ".data");
  if (section == NULL) {
    printf ("Error accessing .text section\n");
    return EXIT_FAILURE;
  }

  /* Configure a MeP disassembler */
  enum bfd_architecture arch = bfd_arch_mep;
  bfd_boolean big_endian = BFD_ENDIAN_BIG;
  unsigned long mach = bfd_mach_mep;
  bfd_default_set_arch_mach(bfdFile, arch, mach);
  disassemble = disassembler (bfd_get_arch (bfdFile), bfd_big_endian (bfdFile),
                              bfd_get_mach (bfdFile), bfdFile);
  if( disassemble == NULL) {
    printf ("Error creating disassembler\n" );
    return EXIT_FAILURE;
  }

  /* Construct and configure the disassembler_info structure */
  init_disassemble_info (&info, stdout, (fprintf_ftype) fprintf);
  info.arch = bfd_get_arch (bfdFile);
  info.mach = bfd_get_mach (bfdFile);
  info.section = section;

  /* Configure buffer information */
  section->vma = 0xC00000; // memory offset
  info.read_memory_func = buffer_read_memory; // builtin
  info.buffer_length = buffer_len;
  info.buffer = (bfd_byte*) &buffer;
  info.buffer_vma = section->vma;

  /* Initialize the disassembler_info structure */
  disassemble_init_for_target (&info);

  /* Diassemble instructions */
  pc = section->vma;
  for (int i=0; i < 4; i++) {
    printf ("Address: 0x%lx\n  ", pc);
    count = disassemble (pc, &info);
    pc += count;
    printf ("\n====\n");
    fflush (stdout);
  }

  return EXIT_SUCCESS;
}
