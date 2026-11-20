{%- if chip == "rp2040" -%}
MEMORY {
    /* The boot ROM expects the second-stage bootloader in the first 256 bytes of flash. */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    /* The Pico and Pico W have 2 MiB of flash. */
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}

SECTIONS {
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;
{%- else -%}
MEMORY {
    /* The Pico 2 and Pico 2 W have 4 MiB of flash. */
    FLASH : ORIGIN = 0x10000000, LENGTH = 4096K
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}

SECTIONS {
    /* The boot ROM looks for the image definition in the first 4 KiB of flash. */
    .start_block : ALIGN(4)
    {
        __start_block_addr = .;
        KEEP(*(.start_block));
        KEEP(*(.boot_info));
    } > FLASH
} INSERT AFTER .vector_table;

_stext = ALIGN(ADDR(.start_block) + SIZEOF(.start_block), ALIGNOF(.text));

SECTIONS {
    .bi_entries : ALIGN(4)
    {
        __bi_entries_start = .;
        KEEP(*(.bi_entries));
        . = ALIGN(4);
        __bi_entries_end = .;
    } > FLASH
} INSERT AFTER .text;

SECTIONS {
    /* Goes after the program, so it can contain a signature. */
    .end_block : ALIGN(4)
    {
        __end_block_addr = .;
        KEEP(*(.end_block));
    } > FLASH
} INSERT AFTER .uninit;

PROVIDE(start_to_end = __end_block_addr - __start_block_addr);
PROVIDE(end_to_start = __start_block_addr - __end_block_addr);
{%- endif %}
