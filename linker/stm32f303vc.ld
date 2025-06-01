/* Linker script for STM32F303VC - 256KB Flash, 40KB RAM */

MEMORY
{
  FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 256K
  RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 40K
}

/* Entry point */
ENTRY(Reset_Handler)

/* Stack and heap size (can be tuned) */
_stack_size = 0x400;
_heap_size  = 0x200;

/* Initial stack pointer */
_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  /* Vector table */
  .isr_vector :
  {
    KEEP(*(.isr_vector))
  } > FLASH

  /* Optional init and fini sections */
  .init :
  {
    KEEP(*(.init))
  } > FLASH

  .fini :
  {
    KEEP(*(.fini))
  } > FLASH

  /* Code + const data */
  .text :
  {
    *(.text*)
    *(.rodata*)
    *(.glue_7)
    *(.glue_7t)
    *(.eh_frame)
    _etext = .;
  } > FLASH

  /* ARM exception unwinding (optional, avoids ld warnings) */
  .ARM.exidx :
  {
    *(.ARM.exidx*)
  } > FLASH

  /* Initialized data: copied from FLASH to RAM */
  .data : AT (_etext)
  {
    _sdata = .;
    _sidata = LOADADDR(.data);
    *(.data*)
    *(.data.*)
    _edata = .;
  } > RAM

  /* Zero-initialized data (BSS) */
  .bss :
  {
    _sbss = .;
    *(.bss*)
    *(COMMON)
    _ebss = .;
  } > RAM

  /* Stack section */
  .stack (COPY) :
  {
    . = ALIGN(8);
    _sstack = .;
    . = . + _stack_size;
    _estack_manual = .;
  } > RAM

  /* Heap section */
  .heap (COPY) :
  {
    . = ALIGN(8);
    _sheap = .;
    . = . + _heap_size;
    _eheap = .;
  } > RAM

  /* Provide aliases for symbols */
  PROVIDE(__stack = _estack);
  PROVIDE(_stack_top = _estack);
  PROVIDE(_heap_start = _sheap);
  PROVIDE(_heap_end   = _eheap);
  PROVIDE(__heap = _sheap);
}
