cmake_minimum_required(VERSION 3.31)
project(simple_digital_synth C CXX ASM)

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR cortex-m4)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 20)

set(LINKER_SCRIPT ${CMAKE_SOURCE_DIR}/stm32/stm32f303vc.ld)

set(SOURCES
#        src/tim6_timebase.c
        src/main.cpp
        src/stm32/stm32_def.c
        src/stm32/system_stm32f3xx.c
#        src/stm32/stm32f3xx_hal_timebase_tim.c
        stm32/startup_stm32f303xc.s
)

SET(CUSTOM_INCLUDE_PATH "${CMAKE_SOURCE_DIR}/../Arduino_Core_STM32-main/system")
SET(CUSTOM_INCLUDE_PATH_2 "${CMAKE_SOURCE_DIR}/../STM32CubeF3-master")

file(GLOB HAL_SOURCES
#        ${CUSTOM_INCLUDE_PATH}/Drivers/STM32F3xx_HAL_Driver/Src/*.c
#        ${CUSTOM_INCLUDE_PATH}/Drivers/STM32F3xx_HAL_Driver/Src/stm32f3xx_hal.c
#        ${CUSTOM_INCLUDE_PATH}/Drivers/STM32F3xx_HAL_Driver/Src/stm32f3xx_hal_rcc.c
#        ${CUSTOM_INCLUDE_PATH}/Drivers/STM32F3xx_HAL_Driver/Src/stm32f3xx_hal_tim.c
#        ${CUSTOM_INCLUDE_PATH}/Drivers/STM32F3xx_HAL_Driver/Src/stm32f3xx_hal_tim_ex.c
#        ${CUSTOM_INCLUDE_PATH}/Drivers/STM32F3xx_HAL_Driver/Src/stm32f3xx_hal_cortex.c
)

add_executable(${PROJECT_NAME}.elf ${SOURCES} ${HAL_SOURCES})

target_compile_definitions(${PROJECT_NAME}.elf PRIVATE STM32F303xC STM32F3xx)

target_include_directories(${PROJECT_NAME}.elf PRIVATE
        ${CMAKE_SOURCE_DIR}/include
        ${CUSTOM_INCLUDE_PATH}/Drivers/CMSIS/Device/ST/STM32F3xx/Include
#        ${CUSTOM_INCLUDE_PATH}/Drivers/CMSIS/Include
#        ${CUSTOM_INCLUDE_PATH}/Drivers/STM32F3xx_HAL_Driver/Inc
#        ${CUSTOM_INCLUDE_PATH}/STM32F3xx
        ${CUSTOM_INCLUDE_PATH_2}/Drivers/CMSIS/Include
#        ${CUSTOM_INCLUDE_PATH_2}/Drivers/CMSIS/Device/ST/STM32F3xx/Include
#        ${CUSTOM_INCLUDE_PATH_2}/Drivers/STM32F3xx_HAL_Driver/Inc
)

target_compile_options(${PROJECT_NAME}.elf PRIVATE
        -mcpu=cortex-m4
        -mthumb
        -g3 -Og
        -ffunction-sections
        -fdata-sections
        -Wall
        -fno-exceptions
        -fno-rtti
)

target_link_options(${PROJECT_NAME}.elf PRIVATE
        -T${LINKER_SCRIPT}
        -mcpu=cortex-m4
        -mthumb
        -Wl,--gc-sections
        -Wl,-Map=${PROJECT_NAME}.map
)

add_custom_command(TARGET ${PROJECT_NAME}.elf POST_BUILD
        COMMAND ${CMAKE_OBJCOPY} -O ihex ${PROJECT_NAME}.elf ${PROJECT_NAME}.hex
        COMMAND ${CMAKE_OBJCOPY} -O binary ${PROJECT_NAME}.elf ${PROJECT_NAME}.bin
)

set(OPENOCD_EXECUTABLE openocd)
set(OPENOCD_CONFIG_FILE ${CMAKE_SOURCE_DIR}/stm32/open-ocd.cfg)
set(FIRMWARE_ELF ${CMAKE_BINARY_DIR}/simple_digital_synth.elf)

add_custom_target(
        flash
        COMMAND ${OPENOCD_EXECUTABLE} -f ${OPENOCD_CONFIG_FILE} -c "program ${FIRMWARE_ELF} verify reset exit"
        DEPENDS ${FIRMWARE_ELF}
        COMMENT "Flashing the firmware to STM32F3 Discovery using OpenOCD"
        VERBATIM
)
