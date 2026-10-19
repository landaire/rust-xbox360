ENTRY(_start)
SECTIONS {
    . = 0x91d01000;
    .text : { *(.text._start) *(.text .text.*) }
    . = ALIGN(4);
    .xthunks : { *(.xthunks .xthunks.*) }
    . = ALIGN(0x1000);
    .rodata : { *(.rodata .rodata.*) }
    . = ALIGN(0x1000);
    .data : { *(.data .data.*) *(.sdata .sdata.*) }
    . = ALIGN(0x1000);
    .bss : { *(.bss .bss.*) *(.sbss .sbss.*) *(COMMON) }
    /DISCARD/ : { *(.eh_frame) *(.eh_frame_hdr) *(.comment) *(.note .note.*) }
}
