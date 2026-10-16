ENTRY(_start)

/* Image base is 0x82000000 (64 KB-aligned, in Xenia's 64 KB-page range). The
   first 64 KB is reserved for the PE headers the packager synthesizes, so code
   starts at 0x82010000. .xthunks sits with .text because the loader patches
   each import thunk in place. */
SECTIONS {
    . = 0x82010000;

    .text : {
        *(.text._start)
        *(.text .text.*)
    }
    . = ALIGN(4);
    .xthunks : { *(.xthunks .xthunks.*) }

    . = ALIGN(0x100);
    .rodata : { *(.rodata .rodata.*) }

    . = ALIGN(0x100);
    .data : { *(.data .data.*) *(.sdata .sdata.*) }
    .xvars : { *(.xvars .xvars.*) }

    . = ALIGN(0x100);
    .bss : { *(.bss .bss.*) *(.sbss .sbss.*) *(COMMON) }

    /DISCARD/ : { *(.eh_frame) *(.eh_frame_hdr) *(.comment) *(.note .note.*) }
}
