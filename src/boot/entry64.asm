    .section .text.entry
    .globl _start
_start:
    # `la rd, symbol` is equal to:
    # auipc rd, symbol[31:12]
    # addi  rd, rd, symbol[11:0]
    # load the address of bootstacktop into sp
    la sp, bootstacktop
    call rust_main

    .section .bss.stack
    .align 12
    .global bootstack
bootstack:
    .space 4096 * 4
    .global bootstacktop
bootstacktop:
