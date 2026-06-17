ENTRY(_start)

MEMORY
{
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128K
}

SECTIONS
{
    .text :
    {
        *(.text*)
        *(.rodata*)
    } > RAM

    .data : ALIGN(8)
    {
        __data_start = .;
        *(.data*)
        __data_end = .;
    } > RAM AT > RAM

    __data_load_start = LOADADDR(.data);

    .bss : ALIGN(8)
    {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    } > RAM

    .stack : ALIGN(16)
    {
        __stack_start = .;
        . = ORIGIN(RAM) + LENGTH(RAM);
        __stack_top = .;
    } > RAM
}