// Guillaume Valadon <guillaume@valadon.net>


#include <stdio.h>
#include <stdlib.h>

#include <include/dis-asm.h>


struct disassemble_info *new_disassemble_info() {
    struct disassemble_info *info = malloc (sizeof(struct disassemble_info));
    return info;
}

void configure_disassemble_info(struct disassemble_info *info, asection *section, bfd *bfdFile) {
    /* Construct and configure the disassembler_info class using stdout */
  
    init_disassemble_info (info, stdout, (fprintf_ftype) fprintf);
    info->arch = bfd_get_arch (bfdFile);
    info->mach = bfd_get_mach (bfdFile);
    info->buffer_vma = section->vma;
    info->buffer_length = section->size;
    info->section = section;
    bfd_malloc_and_get_section (bfdFile, section, &info->buffer);
}


unsigned long get_start_address(bfd *bfdFile) {
  return bfdFile->start_address;
}


unsigned long get_section_size(asection *section) {
  return section->size;
}


void flush_stdout() {
   fflush (stdout);
}


typedef void (*print_address_func) (bfd_vma addr, struct disassemble_info *dinfo);

void set_print_address_func(struct disassemble_info *info,  print_address_func print_function) {
    info->print_address_func = print_function;
    flush_stdout ();
}
