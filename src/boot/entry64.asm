    .equ physical_memory_begin, 0x80000000
    .equ virtual_mapped_begin, 0xffffffffc0000000

    .section .text.entry
    .globl _start
_start:
    # third level page table
    lui t0, %hi(boot_page_table_sv39)
    # the offset between virtual and physical address
    li t1, virtual_mapped_begin - physical_memory_begin
    # get the physical address of the page table
    sub t0, t0, t1
    # get the PPN of page table
    srli t0, t0, 12
    # set MODE to 8 (Sv39)
    li t1, 8 << 60
    # compute the value for satp
    or t0, t0, t1
    csrw satp, t0
    # flush TLB
    sfence.vma
    # now we are in the virtual space!

    # load the address of bootstacktop into sp
    lui sp, %hi(boot_stack_top)
    # call rust_main
    lui t0, %hi(rust_main)
    addi t0, t0, %lo(rust_main)
    jalr t0

    .section .bss.stack
    .align 12
    .global boot_stack
boot_stack:
    .space 4096 * 4
    .global boot_stack_top
boot_stack_top:

    .section .data
    .align 12
boot_page_table_sv39:
    # 0xffff_ffff_c000_0000 maps to 0x8000_0000
    # one huge page of 1GiB
    # we only need the last PTE
    .zero 8 * 511
    # 0x800000 is PPN
    # DAGU XWRV
    # 1100 1111
    .quad (0x80000 << 10) | 0b11001111
