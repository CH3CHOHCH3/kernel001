    .section .text.entry
    .globl _start
 _start:
    li a0, 4096*4
    la sp, boot_stack
    add sp, sp, a0
    # jump to start() in main.rs
    call start

    # set boot stack at the start of .bss
    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096*4
