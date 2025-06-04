#include "stm32f3xx.h"

// Resolution of PWM (ARR = 180 → 0..180 steps)
constexpr uint16_t PWM_RESOLUTION = 180;

volatile uint16_t saw_value = 0;

extern "C" void TIM3_IRQHandler() {
    if (TIM3->SR & TIM_SR_UIF) {
        TIM3->SR &= ~TIM_SR_UIF;  // clear interrupt flag

        // Sawtooth: increment until max, then reset
        saw_value += 1;
        if (saw_value >= PWM_RESOLUTION)
            saw_value = 0;

        TIM2->CCR1 = saw_value;
    }
}

int main() {
    // === GPIOA: enable and configure PA5 to AF1 (TIM2_CH1) ===
    RCC->AHBENR  |= RCC_AHBENR_GPIOAEN;
    GPIOA->MODER &= ~(3U << (5 * 2));
    GPIOA->MODER |=  (2U << (5 * 2));   // Alternate function
    GPIOA->AFR[0] &= ~(0xF << (5 * 4));
    GPIOA->AFR[0] |=  (1U << (5 * 4));  // AF1 = TIM2_CH1

    // === TIM2: PWM output ===
    RCC->APB1ENR |= RCC_APB1ENR_TIM2EN;

    TIM2->PSC = 0;                    // No prescaler
    TIM2->ARR = PWM_RESOLUTION;      // Top value
    TIM2->CCR1 = 0;                  // Start with 0% duty

    TIM2->CCMR1 &= ~TIM_CCMR1_OC1M;
    TIM2->CCMR1 |= (6 << TIM_CCMR1_OC1M_Pos);  // PWM mode 1
    TIM2->CCMR1 |= TIM_CCMR1_OC1PE;            // preload enable
    TIM2->CCER  |= TIM_CCER_CC1E;              // enable output
    TIM2->CR1   |= TIM_CR1_ARPE;               // auto-reload preload enable
    TIM2->EGR   |= TIM_EGR_UG;                 // update event
    TIM2->CR1   |= TIM_CR1_CEN;                // enable timer

    // === TIM3: sample rate timer (44.1 kHz) ===
    RCC->APB1ENR |= RCC_APB1ENR_TIM3EN;

    TIM3->PSC = 0;
    TIM3->ARR = 181 - 1;             // 8 MHz / 181 ≈ 44.2 kHz
    TIM3->DIER |= TIM_DIER_UIE;     // Update interrupt enable
    TIM3->CR1 |= TIM_CR1_CEN;       // Start timer

    NVIC_EnableIRQ(TIM3_IRQn);      // Enable IRQ in NVIC

    while (1) {
        // Nothing to do — all handled in interrupt
    }
}
