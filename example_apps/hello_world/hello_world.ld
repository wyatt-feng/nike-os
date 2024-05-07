SECTIONS {
  .text 0x401000 : { *(.text) }
  .data : { *(.data) } 
  .bss :  { *(.bss)  *(COMMON) }
}
