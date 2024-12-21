ENTRY(_entry)

MEMORY
{
    rom (rx) : ORIGIN = 0x00000000, LENGTH = 0x80000000 /* Set to maximum possible length */
    ram (wrx) : ORIGIN = 0x80000000, LENGTH = 0x80000000 /* Set to maximum possible length */
}

STACK_SIZE = 0x400; /* Default stack size is 1024 bytes */
HEAP_SIZE  = 0x400; /* Default heap size is 1024 bytes */

PHDRS
{
    rom PT_LOAD;
    ram_init PT_LOAD;
    tls PT_TLS;
    ram PT_LOAD;
    text PT_LOAD;
}

SECTIONS
{
    .init : {
        KEEP (*(.text.init.entry))
        KEEP (*(.text.init.*))
        KEEP (*(SORT_NONE(.init)))
    } >rom :rom

    .rodata : {
        *(.rdata)
        *(.rodata .rodata.*)
        *(.gnu.linkonce.r.*)
        . = ALIGN(4);
        *(.srodata.cst16)
        *(.srodata.cst8)
        *(.srodata.cst4)
        *(.srodata.cst2)
        *(.srodata .srodata.*)
    } >rom :rom

    .text : {
        *(.text.unlikely .text.unlikely.*)
        *(.text.startup .text.startup.*)
        *(.text .text.*)
        *(.gnu.linkonce.t.*)
        . = ALIGN(4);
    } >rom :text

    .data : ALIGN(4) {
        *(.data .data.*)
        *(.gnu.linkonce.d.*)
        . = ALIGN(4);
        PROVIDE( __global_pointer$ = . + 0x800 );
        *(.sdata .sdata.* .sdata2.*)
        *(.gnu.linkonce.s.*)
    } >ram AT>rom :ram_init

    .tdata : ALIGN(4) {
        PROVIDE( __tls_base = . );
        *(.tdata .tdata.* .gnu.linkonce.td.*)
    } >ram AT>rom :tls :ram_init

    PROVIDE( __data_source_start = LOADADDR(.data) );
    PROVIDE( __data_target_start = ADDR(.data) );
    PROVIDE( __data_target_end = ADDR(.tdata) + SIZEOF(.tdata) );

    .tbss : ALIGN(4) {
        *(.tbss .tbss.* .gnu.linkonce.tb.*)
        *(.tcommon .tcommon.*)
    } >ram AT>ram :tls :ram

    .tbss_space : ALIGN(4) {
        . = . + SIZEOF(.tbss);
    } >ram :ram

    .bss (NOLOAD): ALIGN(4) {
        *(.sbss*)
        *(.gnu.linkonce.sb.*)
        *(.bss .bss.*)
        *(.gnu.linkonce.b.*)
        *(COMMON)
    } >ram :ram

    PROVIDE( __bss_target_start = ADDR(.tbss) );
    PROVIDE( __bss_target_end = ADDR(.bss) + SIZEOF(.bss) );

    .heap (NOLOAD) : ALIGN(4) {
        PROVIDE(_end = .);
        . += HEAP_SIZE;
        . = ALIGN(16);
    } >ram :ram

    .stack (NOLOAD) : ALIGN(16) {
        . += STACK_SIZE;
        PROVIDE( __stack_start = . );
    } >ram :ram

    /DISCARD/ : {
        *(.eh_frame .eh_frame.*)
    }
}
