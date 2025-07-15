MEMORY
{
    FLASH    : ORIGIN = 0x08100000, LENGTH = 1024K /* BANK_2 */
    RAM      : ORIGIN = 0x10000000, LENGTH = 288K  /* SRAM1, SRAM2, SRAM3 */
    RAM_D3   : ORIGIN = 0x38000000, LENGTH = 64K   /* SRAM4 */
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM); /* = 0x10020000 */

SECTIONS {
    .ram_d3 (NOLOAD) : ALIGN(4) {
        *(.ram_d3 .ram_d3.shared_data .ram_d3.*);
        . = ALIGN(4);
    } > RAM_D3
}
