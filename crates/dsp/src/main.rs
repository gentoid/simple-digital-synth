#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use defmt::info;
use embassy_stm32::{
    SharedData,
    gpio::{Level, Output, Speed},
    hsem::HardwareSemaphore,
    pac,
};
use {defmt_rtt as _, panic_probe as _};

#[unsafe(link_section = ".ram_d3.shared_data")]
static SHARED_DATA: MaybeUninit<SharedData> = MaybeUninit::uninit();

#[cortex_m_rt::entry]
fn main() -> ! {
    pac::RCC.ahb4enr().modify(|w| w.set_hsemen(true));

    let p = unsafe { embassy_stm32::Peripherals::steal() };
    let hsem = HardwareSemaphore::new(p.HSEM);

    mcu_common::hsem::init_hsem_driver(hsem);
    critical_section::set_impl!(mcu_common::hsem::HsemCriticalSection);

    info!("M7 core started!");

    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;

        config.rcc.hsi = Some(HSIPrescaler::DIV1);
        config.rcc.csi = true;
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            divq: Some(PllDiv::DIV8),
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.ahb_pre = AHBPrescaler::DIV2;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.apb3_pre = APBPrescaler::DIV2;
        config.rcc.apb4_pre = APBPrescaler::DIV2;
        config.rcc.voltage_scale = VoltageScale::Scale1;
        config.rcc.supply_config = SupplyConfig::DirectSMPS;

        config.enable_debug_during_sleep = true;
    }

    let p = embassy_stm32::init_primary(config, &SHARED_DATA);
    
    pac::RCC.c1_ahb4enr().modify(|w| w.set_gpioeen(true));

    pac::RCC.gcr().modify(|w| w.set_boot_c2(true));

    let mc4_booted = pac::RCC.gcr().modify(|w| w.boot_c2());
    info!("M7: CM4 boot enabled: {}", mc4_booted);

    info!("M7: Embassy STM32 initialized!");

    let mut led1 = Output::new(p.PB14, Level::High, Speed::Low);
    let mut led2 = Output::new(p.PE1, Level::High, Speed::Low);
    let mut led3 = Output::new(p.PB0, Level::High, Speed::Low);

    loop {
        // cortex_m::asm::nop();
        info!("High");
        led1.set_high();
        cortex_m::asm::delay(1_000_00000);
        // led2.set_high();
        cortex_m::asm::delay(1_000_00000);
        led3.set_high();
        cortex_m::asm::delay(1_000_00000);
        info!("Low");
        led1.set_low();
        cortex_m::asm::delay(1_000_00000);
        // led2.set_low();
        cortex_m::asm::delay(1_000_00000);
        led3.set_low();
        cortex_m::asm::delay(1_000_00000);
    }
}

// #[cortex_m_rt::interrupt]
// fn TIM2() {
//     free(|cs| {
//         if let Some(ref mut led) = LED.borrow(cs).borrow_mut().as_mut() {
//             led.toggle();
//         }
//     });
//     // Clear interrupt flag
//     let dp = unsafe { pac::Peripherals::steal() };
//     dp.TIM2.sr.modify(|_, w| w.uif().clear_bit());
// }
