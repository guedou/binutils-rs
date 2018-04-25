// Guillaume Valadon <guillaume@valadon.net>

#include <config.h>

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <bfd.h>
#include <dis-asm.h>

// Silly macro that helps removing the unused warnings
#define UNUSED_VARIABLE(id) id=id

#define R_ASM_BUFSIZE 64
char tmp_buf_asm[R_ASM_BUFSIZE];
char *tmp_buf_asm_ptr = tmp_buf_asm;
void copy_buffer(void * useless, const char* format, ...) {
    UNUSED_VARIABLE(useless);

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


// TODO: must be in Rust!
bfd_byte buffer[] = { 0xc3, 0x90, 0x66, 0x90 };
unsigned int buffer_len = 4;
int buffer_ptr = 0;

int pouet(bfd_vma memaddr, bfd_byte *myaddr, unsigned int length, struct disassemble_info *info) {
    UNUSED_VARIABLE(memaddr);
    UNUSED_VARIABLE(info);

    if (length > buffer_len)
      return 1;

    memcpy (myaddr, &buffer[buffer_ptr], length);
    buffer_ptr += length;

    return 0;
}

void configure_disassemble_info_buffer(struct disassemble_info *info, enum bfd_architecture arch, unsigned long mach) {
//typedef int (*copy_buffer_ptr) (bfd_vma memaddr, bfd_byte *myaddr, unsigned int length, struct disassemble_info *dinfo);
//void configure_disassemble_info_buffer(struct disassemble_info *info, enum bfd_architecture arch, unsigned long mach, copy_buffer_ptr copy_function) {
  
    init_disassemble_info (info, stdout, (fprintf_ftype) copy_buffer);
    info->arch = arch;
    info->mach = mach;
    info->read_memory_func = buffer_read_memory;
    //info->read_memory_func = copy_function;
}


unsigned long get_start_address(bfd *bfdFile) {
    return bfdFile->start_address;
}


unsigned long get_section_size(asection *section) {
    return section->size;
}

typedef void (*print_address_func) (bfd_vma addr, struct disassemble_info *dinfo);

void set_print_address_func(struct disassemble_info *info, print_address_func print_function) {
    info->print_address_func = print_function;
}


unsigned int call_bfd_big_endian(bfd *bfdFile) {
    return bfd_big_endian(bfdFile);
}

void set_buffer(struct disassemble_info *info, bfd_byte* buffer, unsigned int length, bfd_vma vma) {
    info->buffer = buffer;
    info->buffer_length = length;
    info->buffer_vma = vma;

    asection section;
    info->section = &section;
}


void show_buffer(struct disassemble_info *info) {
    printf("len=%d - vma=%lu\n", info->buffer_length, info->buffer_vma);
    printf("%p\n", info->buffer);
    printf("%x\n", info->buffer[0]);
    printf("%x\n", info->buffer[1]);
    printf("%x\n", info->buffer[2]);
    printf("%x\n", info->buffer[3]);
}


enum bfd_architecture get_arch(struct bfd_arch_info *arch_info) {
  return arch_info->arch;
}
