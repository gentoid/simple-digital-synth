#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use defmt::info;
use embassy_stm32::{
    SharedData,
    gpio::{Level, Output, Speed},
    hsem::HardwareSemaphore,
};
use {defmt_rtt as _, panic_probe as _};

#[unsafe(link_section = ".ram_d3.shared_data")]
static SHARED_DATA: MaybeUninit<SharedData> = MaybeUninit::uninit();

#[cortex_m_rt::entry]
fn main() -> ! {
    // let p = unsafe { embassy_stm32::Peripherals::steal() };
    // let cpuid = unsafe { cortex_m::peripheral::CPUID::PTR.read_volatile().base.read() };
    // let id = cpuid & 0xF0;
    // // info!("Id is {}", id);
    // let hsem = HardwareSemaphore::new(p.HSEM);
    // mcu_common::hsem::init_hsem_driver(hsem);
    // critical_section::set_impl!(mcu_common::hsem::HsemCriticalSection);

    // for i in 0..id {
    //     info!("Hello {}", i);
    // }

    // info!("M7 core started!");

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
    }

    let p = embassy_stm32::init_primary(config, &SHARED_DATA);

    info!("Embassy STM32 initialized!");

    // embassy_stm32::pac::PWR.cpucr().modify(|w| w.set_cssf(val));

    // let rcc = p.RCC;

    // rcc
    // unsafe {
    //     p.
    // }

    let mut led = Output::new(p.PB14, Level::High, Speed::Low);

    loop {
        info!("High");
        led.set_high();
        cortex_m::asm::delay(4_000_00000);
        info!("Low");
        led.set_low();
        cortex_m::asm::delay(4_000_00000);
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
