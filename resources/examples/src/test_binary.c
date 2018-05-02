// Guillaume Valadon <guillaume@valadon.net>

#include <config.h>

#include <stdlib.h>
#include <string.h>

#include <bfd.h>
#include <dis-asm.h>

void override_print_address(bfd_vma addr, struct disassemble_info *info)
{
  // This function change how addresses are displayed
  printf ("0x%x", addr);
}

int main(int argc, char **argv)
{
  bfd                     *bfdFile;
  asection                *section;
  disassembler_ftype      disassemble;
  struct disassemble_info info;
  unsigned long           count, pc;

  /* Open an ELF binary with libbfd */
  bfd_init ();
  bfdFile = bfd_openr ("/bin/ls", "elf64-x86-64");
  if (bfdFile == NULL) {
    printf ("Error [%x]: %s\n", bfd_get_error (),
                                bfd_errmsg (bfd_get_error ()));
    return EXIT_FAILURE;
  }
  if (!bfd_check_format( bfdFile, bfd_object)) {
    printf ("Error [%x]: %s\n", bfd_get_error (),
                                bfd_errmsg (bfd_get_error ()));
    return EXIT_FAILURE;
  }

  /* Retrieve the ELF .text code section */
  section = bfd_get_section_by_name (bfdFile, ".text");
  if (section == NULL) {
    printf ("Error accessing .text section\n");
    return EXIT_FAILURE;
  }

  /* Get a disassemble function pointer */
  disassemble = disassembler (bfd_get_arch (bfdFile), bfd_big_endian (bfdFile),
                              bfd_get_mach (bfdFile), bfdFile);
  if (disassemble == NULL) {
    printf ("Error creating disassembler\n");
    return EXIT_FAILURE;
  }

  /* Construct and configure the disassembler_info class */ 
  init_disassemble_info (&info, stdout, (fprintf_ftype) fprintf);
  info.print_address_func = override_print_address;
  info.arch          = bfd_get_arch (bfdFile);
  info.mach          = bfd_get_mach (bfdFile);
  info.buffer_vma    = section->vma;
  info.buffer_length = section->size;
  info.section       = section;
  bfd_malloc_and_get_section (bfdFile, section, &info.buffer);
  disassemble_init_for_target (&info);

  /* Start diassembling */
  pc = bfd_get_start_address (bfdFile);
  do {
    printf ("0x%x  ", pc);
    count = disassemble (pc, &info);
    pc += count;
    printf ("\n");
  } while (count > 0 && pc <= section->size);

  return EXIT_SUCCESS;
}
