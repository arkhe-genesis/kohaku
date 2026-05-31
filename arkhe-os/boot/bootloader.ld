/* ARKHE OS Bootloader Linker Script */
ENTRY(_start)

SECTIONS {
    . = 0x7C00; /* Standard bootloader load address, adjust if UEFI */

    .boot : {
        *(.boot)
    }

    .text : {
        *(.text)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
    }

    /* Boot signature for legacy BIOS */
    . = 0x7DFE;
    .signature : {
        SHORT(0xAA55)
    }
}
