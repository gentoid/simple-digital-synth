#include "stm32f3xx_hal.h"
#include <cstdint>

class LedRing {
public:
  static constexpr uint16_t leds_mask = 0xFF00; // PE8..PE15

  void init() {
    __HAL_RCC_GPIOE_CLK_ENABLE();

    GPIOE->MODER &= ~(0xFFFF << (8 * 2));
    GPIOE->MODER |=
        (0x5555 << (8 * 2));
  }

  void set(uint8_t pattern) {
    GPIOE->BSRR = ((~pattern & 0xFF) << 24) | ((pattern & 0xFF) << 8);
  }
};

void SystemClock_Config() {
  RCC_OscInitTypeDef RCC_OscInitStruct = {};
  RCC_ClkInitTypeDef RCC_ClkInitStruct = {};

  RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSI;
  RCC_OscInitStruct.HSIState = RCC_HSI_ON;
  RCC_OscInitStruct.HSICalibrationValue = RCC_HSICALIBRATION_DEFAULT;
  RCC_OscInitStruct.PLL.PLLState = RCC_PLL_NONE;
  HAL_RCC_OscConfig(&RCC_OscInitStruct);

  RCC_ClkInitStruct.ClockType = RCC_CLOCKTYPE_HCLK | RCC_CLOCKTYPE_SYSCLK
                              | RCC_CLOCKTYPE_PCLK1 | RCC_CLOCKTYPE_PCLK2;
  RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_HSI;
  RCC_ClkInitStruct.AHBCLKDivider = RCC_SYSCLK_DIV1;
  RCC_ClkInitStruct.APB1CLKDivider = RCC_HCLK_DIV1;
  RCC_ClkInitStruct.APB2CLKDivider = RCC_HCLK_DIV1;
  HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_ACR_LATENCY_0);
}

int main() {
  HAL_Init();
  SystemClock_Config();

  LedRing leds;
  leds.init();

  uint8_t pattern = 0x01;

  while (true) {
    leds.set(pattern);
    pattern = (pattern << 1) | (pattern >> 7);

    HAL_Delay(500);
  }
}
