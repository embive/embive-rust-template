MEMORY
{
	FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x80000000 /* Set to maximum possible length */
	RAM (xrw)  : ORIGIN = 0x80000000, LENGTH = 0x80000000 /* Set to maximum possible length */
}

SECTIONS
{
    .entry :
    { 
      KEEP(*(SORT_NONE(.entry)))
      . = ALIGN(4);
    } >FLASH AT>FLASH

    .text :
    {
      *(.text)
      *(.text.*)
      PROVIDE( _srodata = .);
      *(.rodata)
      *(.rodata*)
      . = ALIGN(4);
    } >FLASH AT>FLASH 

    .data :
    {
      PROVIDE( _sdata = .);
      *(.data .data.*)
      . = ALIGN(4);
      PROVIDE( _edata = .);
    } >RAM AT>FLASH

    .bss :
    {
      PROVIDE( _sbss = .);
      *(.sbss*)
      *(.bss*)
      . = ALIGN(4);
      PROVIDE( _ebss = .);
    } >RAM AT>FLASH
}
