OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    . = 0x80200000;             /* start of the kernel */

    .text : {                   /* .text will be the first section */
        *(.text.entry)          /* .text.entry is the start */
        *(.text .text.*)
        etext = .;              /* end of .text */
    }

    .rodata : {
        . = ALIGN(4K);          /* another page for .rodata section */
        srodata = .;            /* start of .rodata */
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        erodata = .;            /* end of .rodata */
    }

    .data : {
        . = ALIGN(4K);
        sdata = .;               /* start of .data */
        *(.data .data.*)
        *(.sdata .sdata.*)
        edata = .;               /* end of .data */
    }

    .bss : {
        . = ALIGN(4K);
        *(.bss.stack)
        sbss = .;               /* start of .bss */
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;               /* end of .bss */
    }

    . = ALIGN(4K);
    ekernel = .;                /* end of the kernel */
}
