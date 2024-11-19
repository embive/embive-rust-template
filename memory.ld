ENTRY(_entry)

MEMORY
{
    rom (rx) : ORIGIN = 0x00000000, LENGTH = 0x80000000 /* Set to maximum possible length */
    ram (wrx) : ORIGIN = 0x80000000, LENGTH = 0x80000000 /* Set to maximum possible length */
}

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
    } >rom :text

    .data : ALIGN(4) {
        *(.data .data.*)
        *(.gnu.linkonce.d.*)
        . = ALIGN(4);
        *(.sdata .sdata.* .sdata2.*)
        *(.gnu.linkonce.s.*)
    } >ram AT>rom :ram_init

    .tdata : ALIGN(4) {
        PROVIDE( __tls_base = . );
        *(.tdata .tdata.* .gnu.linkonce.td.*)
    } >ram AT>rom :tls :ram_init

    PROVIDE( _data_source_start = LOADADDR(.data) );
    PROVIDE( _data_target_start = ADDR(.data) );
    PROVIDE( _data_target_end = ADDR(.tdata) + SIZEOF(.tdata) );

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

    PROVIDE( _bss_target_start = ADDR(.tbss) );
    PROVIDE( _bss_target_end = ADDR(.bss) + SIZEOF(.bss) );

    /DISCARD/ : {
	    *(.eh_frame .eh_frame.*)
    }
}