OUTPUT_FORMAT("elf64-littleaarch64")

ENTRY(_start)

SECTIONS
{
    . = 0x1000;

    .init_info ALIGN(0x1000) :
    {
    }

    .text ALIGN(0x1000) :
    {
        *(.text);
        *(.text.*)
    }

    .rodata ALIGN(0x1000) :
    {
        *(.rodata);
        *(.rodata.*);
    }

    .data ALIGN(0x1000) :
    {
        *(.data);
        *(.data.*);
    }

    .bss ALIGN(0x1000) :
    {
        __init_info_start = .;
        . += 0x2000;
        __init_info_end = .;

        __init_ipc_buffer_start = .;
        . += 0x1000;
        __init_ipc_buffer_end = .;

        __init_stack_start = .;
        . += 0x200000;
        __init_stack_end = .;

        __bss_start = .;
        *(.bss);
        *(.bss.*);
        __bss_end = .;
    }

    . = 0x2000000000;

    /DISCARD/ :
    {
        *(*.eh_frame);
    }
}
