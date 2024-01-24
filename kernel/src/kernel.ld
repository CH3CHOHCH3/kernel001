OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    . = 0x80200000;             /* start of the kernel */

    .text : {                   /* .text will be the first section */
        *(.text.entry)          /* .text.entry is at the bottom */
        *(.text .text.*)
        PROVIDE(etext = .);     /* the end of .text */
    }

    .rodata : {
        . = ALIGN(4K);          /* another page for .rodata section */
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    .data : {
        . = ALIGN(4K);
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .bss : {
        . = ALIGN(4K);
        *(.bss.stack)
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    
    . = ALIGN(4K);
    PROVIDE(ekernel = .);       /* end of the kernel */
}
