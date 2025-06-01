#pragma once

#ifndef PROJ_HAL_CONF_H
#define PROJ_HAL_CONF_H

#ifdef __cplusplus
extern "C" {
#endif

  /* ########################## Module Selection ############################## */
#define HAL_MODULE_ENABLED
#define HAL_GPIO_MODULE_ENABLED
#define HAL_RCC_MODULE_ENABLED
#define HAL_CORTEX_MODULE_ENABLED
#define HAL_FLASH_MODULE_ENABLED
#define HAL_DMA_MODULE_ENABLED
#define HAL_RTC_MODULE_ENABLED
#define HAL_PWR_MODULE_ENABLED
#define HAL_TIM_MODULE_ENABLED

  /* ########################## HSE/HSI Values adaptation ##################### */
#define HSE_VALUE    ((uint32_t)8000000U)  /*!< Value of the External oscillator in Hz */
#define HSI_VALUE    ((uint32_t)8000000U)  /*!< Value of the Internal oscillator in Hz */
#define HSI48_VALUE  ((uint32_t)48000000U) /*!< Value of the Internal High Speed oscillator for USB FS */

#define LSI_VALUE    ((uint32_t)40000U)    /*!< LSI Typical Value in Hz */
#define LSE_VALUE    ((uint32_t)32768U)    /*!< Value of the External Low Speed oscillator in Hz */

#define LSE_STARTUP_TIMEOUT    (5000U)   /*!< Time out for LSE start up, in ms */
#define HSE_STARTUP_TIMEOUT    (100U)   /*!< Time out for HSE start up, in ms */

  /* ########################### System Configuration ######################### */
#define  VDD_VALUE                      ((uint32_t)3300U)
#define  TICK_INT_PRIORITY              ((uint32_t)0U)   /*!< tick interrupt priority */
#define  USE_RTOS                       0
#define  PREFETCH_ENABLE                1
#define  INSTRUCTION_CACHE_ENABLE       1
#define  DATA_CACHE_ENABLE              1

  /* ########################## Assert Selection ############################## */
  /* Uncomment to enable full assert */
  #define USE_FULL_ASSERT    1U

  /* Includes ------------------------------------------------------------------*/
#include "stm32_assert.h"
#include "stm32f3xx_hal_def.h"
#include "stm32f3xx_hal_gpio.h"
#include "stm32f3xx_hal_rcc.h"
#include "stm32f3xx_hal_cortex.h"
#include "stm32f3xx_hal_flash.h"
#include "stm32f3xx_hal_dma.h"
#include "stm32f3xx_hal_rtc.h"
#include "stm32f3xx_hal_pwr.h"
#include "stm32f3xx_hal_tim.h"

#ifdef __cplusplus
}
#endif

#endif
