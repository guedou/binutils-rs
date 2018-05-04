// Guillaume Valadon <guillaume@valadon.net>
// C based binutils and custom helpers

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
char buffer_asm[R_ASM_BUFSIZE];
char *buffer_asm_ptr = buffer_asm;
void copy_buffer(void * useless, const char* format, ...) {
    UNUSED_VARIABLE(useless);

    // TODO: handle this in Rust?
    va_list ap;

    va_start(ap, format);
    vsnprintf(buffer_asm_ptr, R_ASM_BUFSIZE-(buffer_asm_ptr-buffer_asm), format, ap);
    buffer_asm_ptr = buffer_asm + strlen(buffer_asm);
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


void configure_disassemble_info_buffer(struct disassemble_info *info, enum bfd_architecture arch, unsigned long mach) {
  
    init_disassemble_info (info, stdout, (fprintf_ftype) copy_buffer);
    info->arch = arch;
    info->mach = mach;
    info->read_memory_func = buffer_read_memory;
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


unsigned int macro_bfd_big_endian(bfd *bfdFile) {
    return bfd_big_endian(bfdFile);
}

void set_buffer(struct disassemble_info *info, bfd_byte* buffer, unsigned int length, bfd_vma vma) {
    info->buffer = buffer;
    info->buffer_length = length;
    info->buffer_vma = vma;

    asection *section = malloc(sizeof(asection));
    info->section = section;
    info->section->vma = vma;
}

void free_disassemble_info(struct disassemble_info *info) {
  free(info->section);
  free(info);
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

unsigned long get_mach(struct bfd_arch_info *arch_info) {
  return arch_info->mach;
}


void mep_disassemble_info(struct disassemble_info* info) {
  // From GDB
  info->octets_per_byte = 1;
  info->skip_zeroes = 256;
  info->skip_zeroes_at_end = 0;
  info->insn_type = dis_noninsn;
  info->target = 0;
  info->target2 = 0;
  info->stop_vma = 0;
  info->flags = DISASSEMBLE_DATA;

  info->section->flags = 0; // GV: important grep VLIW in dis-mep.c to know why
}


unsigned long get_disassemble_info_section_vma(struct disassemble_info *info) {
  return info->section->vma;
}
