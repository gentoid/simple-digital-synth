arm-none-eabi-objdump -h target/thumbv7em-none-eabihf/debug/main


target/thumbv7em-none-eabihf/debug/main:     file format elf32-littlearm

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .vector_table 00000194  08000000  08000000  00010000  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  1 .text         000001a0  08000194  08000194  00010194  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  2 .rodata       0000002c  08000334  08000334  00010334  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  3 .data         00000000  20000000  20000000  00010360  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  4 .gnu.sgstubs  00000000  08000360  08000360  00010360  2**5
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  5 .bss          00000004  20000000  20000000  00020000  2**2
                  ALLOC
  6 .uninit       00000000  20000004  20000004  00020000  2**2
                  ALLOC
  7 .debug_loc    000002ba  00000000  00000000  00020000  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  8 .debug_abbrev 000007d0  00000000  00000000  000202ba  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  9 .debug_info   0000793f  00000000  00000000  00020a8a  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
 10 .debug_aranges 00000228  00000000  00000000  000283c9  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
 11 .debug_ranges 00000190  00000000  00000000  000285f1  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
 12 .debug_str    000089d3  00000000  00000000  00028781  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
 13 .comment      00000099  00000000  00000000  00031154  2**0
                  CONTENTS, READONLY
 14 .ARM.attributes 00000038  00000000  00000000  000311ed  2**0
                  CONTENTS, READONLY
 15 .debug_frame  00000310  00000000  00000000  00031228  2**2
                  CONTENTS, READONLY, DEBUGGING, OCTETS
 16 .debug_line   00001250  00000000  00000000  00031538  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
 17 .debug_pubnames 000002be  00000000  00000000  00032788  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
 18 .debug_pubtypes 00000047  00000000  00000000  00032a46  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS




arm-none-eabi-nm --defined-only .\target\thumbv7em-none-eabihf\debug\main

08000334 r .Lanon.669c0cbbf1a03f348c2b58ac7ea819d2.222
08000322 T __cpsid
08000326 T __cpsie
20000004 B __ebss
20000000 R __edata
08000040 R __eexceptions
08000360 R __erodata
08000334 T __etext
20000004 B __euninit
08000008 R __exceptions
08000008 R __EXCEPTIONS
08000040 R __INTERRUPTS
0800031e T __pre_init
0800032a T __primask_r
08000004 R __RESET_VECTOR
20000000 B __sbss
20000000 R __sdata
20000004 B __sheap
08000360 A __sidata
08000334 R __srodata
08000194 T __stext
20000004 B __suninit
08000000 R __vector_table
08000360 R __veneer_base
08000360 R __veneer_limit
2000a000 A _ram_end
20000004 B _stack_end
2000a000 A _stack_start
08000194 R _stext
0800030a t _ZN4core6option13unwrap_failed17h50f902983f39afffE
08000302 t _ZN4core9panicking5panic17hb7da25e41ead6f4cE
080002fa t _ZN4core9panicking9panic_fmt17he614eca46fdfef54E
080001f4 t _ZN4main18__cortex_m_rt_main17hf452e1f54a1ce6ebE
20000000 b _ZN8cortex_m10peripheral5TAKEN17h374daec91914aa8dE.0
080002ec t _ZN8cortex_m8register7primask4read17h37d83d3c0cb82969E
0800031c T ADC1_2
0800031c T ADC3
0800031c T ADC4
0800031c T BusFault
0800031c T CAN_RX1
0800031c T CAN_SCE
0800031c T COMP1_2_3
0800031c T COMP4_5_6
0800031c T COMP7
0800031c T DebugMonitor
0800031c T DefaultHandler
0800031c T DefaultHandler_
0800031e T DefaultPreInit
20000001 B DEVICE_PERIPHERALS
0800031c T DMA1_CH1
0800031c T DMA1_CH2
0800031c T DMA1_CH3
0800031c T DMA1_CH4
0800031c T DMA1_CH5
0800031c T DMA1_CH6
0800031c T DMA1_CH7
0800031c T DMA2_CH1
0800031c T DMA2_CH2
0800031c T DMA2_CH3
0800031c T DMA2_CH4
0800031c T DMA2_CH5
0800031c T EXTI0
0800031c T EXTI1
0800031c T EXTI15_10
0800031c T EXTI2_TSC
0800031c T EXTI3
0800031c T EXTI4
0800031c T EXTI9_5
0800031c T FLASH
0800031c T FMC
0800031c T FPU
08000330 T HardFault
08000330 T HardFault_
0800031c T I2C1_ER
0800031c T I2C1_EV_EXTI23
0800031c T I2C2_ER
0800031c T I2C2_EV_EXTI24
0800031c T I2C3_ER
0800031c T I2C3_EV
080001ec T main
0800031c T MemoryManagement
0800031c T NonMaskableInt
0800031c T PendSV
0800031c T PVD
0800031c T RCC
08000194 T Reset
0800031c T RTC_WKUP
0800031c T RTCALARM
08000320 t rust_begin_unwind
0800031c T SPI1
0800031c T SPI2
0800031c T SPI3
0800031c T SPI4
0800031c T SVCall
0800031c T SysTick
0800031c T TAMP_STAMP
0800031c T TIM1_BRK_TIM15
0800031c T TIM1_CC
0800031c T TIM1_TRG_COM_TIM17
0800031c T TIM1_UP_TIM16
0800031c T TIM2
0800031c T TIM20_BRK
0800031c T TIM20_CC
0800031c T TIM20_TRG_COM
0800031c T TIM20_UP
0800031c T TIM3
0800031c T TIM4
0800031c T TIM6_DACUNDER
0800031c T TIM7
0800031c T TIM8_BRK
0800031c T TIM8_CC
0800031c T TIM8_TRG_COM
0800031c T TIM8_UP
0800031c T UART4_EXTI34
0800031c T UART5_EXTI35
0800031c T UsageFault
0800031c T USART1_EXTI25
0800031c T USART2_EXTI26
0800031c T USART3_EXTI28
0800031c T USB_HP
0800031c T USB_HP_CAN_TX
0800031c T USB_LP
0800031c T USB_LP_CAN_RX0
0800031c T USB_WKUP
0800031c T USB_WKUP_EXTI
0800031c T WWDG


cargo readobj --target thumbv7em-none-eabihf --bin main -- --file-header

    Finished `dev` profile [optimized + debuginfo] target(s) in 0.33s
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x8000195
  Start of program headers:          52 (bytes into file)
  Start of section headers:          211688 (bytes into file)
  Flags:                             0x5000400
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         5
  Size of section headers:           40 (bytes)
  Number of section headers:         23
  Section header string table index: 21