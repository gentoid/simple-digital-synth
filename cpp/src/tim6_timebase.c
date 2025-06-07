#include "stm32f3xx_hal.h"

static TIM_HandleTypeDef TimHandle;

HAL_StatusTypeDef HAL_InitTick(uint32_t TickPriority)
{
    RCC_ClkInitTypeDef clkconfig;
    uint32_t uwTimclock, uwPrescalerValue, pFLatency;
    uint32_t uwAPB1Prescaler = 0U;

    __HAL_RCC_TIM6_CLK_ENABLE();

    HAL_RCC_GetClockConfig(&clkconfig, &pFLatency);
    uwAPB1Prescaler = clkconfig.APB1CLKDivider;

    if (uwAPB1Prescaler == RCC_HCLK_DIV1)
        uwTimclock = HAL_RCC_GetPCLK1Freq();
    else
        uwTimclock = 2U * HAL_RCC_GetPCLK1Freq();

    uwPrescalerValue = (uwTimclock / 1000000U) - 1U;

    TimHandle.Instance = TIM6;
    TimHandle.Init.Period = (1000000U / 1000U) - 1U;
    TimHandle.Init.Prescaler = uwPrescalerValue;
    TimHandle.Init.ClockDivision = 0;
    TimHandle.Init.CounterMode = TIM_COUNTERMODE_UP;
    TimHandle.Init.AutoReloadPreload = TIM_AUTORELOAD_PRELOAD_DISABLE;

    if (HAL_TIM_Base_Init(&TimHandle) != HAL_OK)
        return HAL_ERROR;

    if (HAL_TIM_Base_Start_IT(&TimHandle) != HAL_OK)
        return HAL_ERROR;

    HAL_NVIC_SetPriority(TIM6_DAC_IRQn, TickPriority, 0U);
    HAL_NVIC_EnableIRQ(TIM6_DAC_IRQn);

    return HAL_OK;
}

void TIM6_DAC_IRQHandler(void)
{
    HAL_TIM_IRQHandler(&TimHandle);
}

void HAL_TIM_PeriodElapsedCallback(TIM_HandleTypeDef *htim)
{
    if (htim->Instance == TIM6) {
        HAL_IncTick();
    }
}
