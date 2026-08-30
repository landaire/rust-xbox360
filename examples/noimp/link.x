ENTRY(_start)
SECTIONS {
    . = 0x82010000;
    .text : { *(.text._start) *(.text .text.*) }
    . = ALIGN(0x10000);
    .rodata : { *(.rodata .rodata.*) }
    . = ALIGN(0x10000);
    .data : { *(.data .data.*) *(.sdata .sdata.*) }
    . = ALIGN(0x10000);
    .bss : { *(.bss .bss.*) *(.sbss .sbss.*) *(COMMON) }
    /DISCARD/ : { *(.eh_frame) *(.eh_frame_hdr) *(.comment) *(.note .note.*) }
}
