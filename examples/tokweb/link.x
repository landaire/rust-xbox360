ENTRY(_start)

/* Image base 0x82000000; first 64 KB reserved for PE headers, code at
   0x82010000. Sections are 64 KB-aligned so each page has uniform attributes
   (required by real hardware). .xthunks sits on the executable .text page
   because the loader patches each import thunk in place. */
SECTIONS {
    . = 0x82010000;
    .text : {
        *(.text._start)
        *(.text .text.*)
    }
    . = ALIGN(4);
    .xthunks : { *(.xthunks .xthunks.*) }

    . = ALIGN(0x10000);
    .rodata : { *(.rodata .rodata.*) }

    . = ALIGN(0x10000);
    .data : { *(.data .data.*) *(.sdata .sdata.*) }

    . = ALIGN(0x10000);
    .bss : { *(.bss .bss.*) *(.sbss .sbss.*) *(COMMON) }

    /DISCARD/ : { *(.eh_frame) *(.eh_frame_hdr) *(.comment) *(.note .note.*) }
}
