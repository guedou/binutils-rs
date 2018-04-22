// Guillaume Valadon <guillaume@valadon.net>


#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <include/dis-asm.h>


#define R_ASM_BUFSIZE 64
char tmp_buf_asm[R_ASM_BUFSIZE];
char *tmp_buf_asm_ptr = tmp_buf_asm;
void copy_buffer(void * useless, const char* format, ...) {
    // TODO: handle this in Rust?
    va_list ap;

    va_start(ap, format);
    vsnprintf(tmp_buf_asm_ptr, R_ASM_BUFSIZE-(tmp_buf_asm_ptr-tmp_buf_asm), format, ap);
    tmp_buf_asm_ptr = tmp_buf_asm + strlen(tmp_buf_asm);
    va_end(ap);
}

struct disassemble_info *new_disassemble_info() {
    struct disassemble_info *info = malloc (sizeof(struct disassemble_info));
    return info;
}

void configure_disassemble_info(struct disassemble_info *info, asection *section, bfd *bfdFile) {
    /* Construct and configure the disassembler_info class using stdout */
  
    init_disassemble_info (info, stdout, (fprintf_ftype) copy_buffer);
    info->arch = bfd_get_arch (bfdFile);
    info->mach = bfd_get_mach (bfdFile);
    info->section = section;

    info->buffer_vma = section->vma;
    info->buffer_length = section->size;

    bfd_malloc_and_get_section (bfdFile, section, &info->buffer);
}


unsigned long get_start_address(bfd *bfdFile) {
  return bfdFile->start_address;
}


unsigned long get_section_size(asection *section) {
  return section->size;
}

typedef void (*print_address_func) (bfd_vma addr, struct disassemble_info *dinfo);

void set_print_address_func(struct disassemble_info *info,  print_address_func print_function) {
    info->print_address_func = print_function;
}
