#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use defmt::info;
use embassy_stm32::{
    SharedData,
    gpio::{Level, Output, Speed},
    hsem::HardwareSemaphore, pac,
};
use {defmt_rtt as _, panic_probe as _};

#[unsafe(link_section = ".ram_d3.shared_data")]
static SHARED_DATA: MaybeUninit<SharedData> = MaybeUninit::uninit();

#[cortex_m_rt::entry]
fn main() -> ! {
    pac::RCC.ahb4enr().modify(|w| w.set_hsemen(true));
    let p = unsafe { embassy_stm32::Peripherals::steal() };
    let hsem = HardwareSemaphore::new(p.HSEM);
    // let clear_key = hsem.get_clear_key();
    // let int0 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 0);
    // let int1 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 1);
    // let int2 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 2);
    // let int3 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 3);
    // let int4 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 4);
    // let int5 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 5);
    // let int6 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 6);
    // let int7 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 7);
    // let int8 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 8);
    // let int9 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 9);
    // let int10 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 10);
    // let int11 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 11);
    // let int12 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 12);
    // let int13 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 13);
    // let int14 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 14);
    // let int15 = hsem.is_interrupt_active(embassy_stm32::hsem::CoreId::Core0, 15);

    // let lock0 = hsem.is_semaphore_locked(0);
    // let lock1 = hsem.is_semaphore_locked(1);
    // let lock2 = hsem.is_semaphore_locked(2);
    // let lock3 = hsem.is_semaphore_locked(3);
    // let lock4 = hsem.is_semaphore_locked(4);
    // let lock5 = hsem.is_semaphore_locked(5);
    // let lock6 = hsem.is_semaphore_locked(6);
    // let lock7 = hsem.is_semaphore_locked(7);
    // let lock8 = hsem.is_semaphore_locked(8);
    // let lock9 = hsem.is_semaphore_locked(9);
    // let lock10 = hsem.is_semaphore_locked(10);
    // let lock11 = hsem.is_semaphore_locked(11);
    // let lock12 = hsem.is_semaphore_locked(12);
    // let lock13 = hsem.is_semaphore_locked(13);
    // let lock14 = hsem.is_semaphore_locked(14);
    // let lock15 = hsem.is_semaphore_locked(15);

    // let sem1 = hsem.two_step_lock(1, 0);
    // let ee = hsem.is_semaphore_locked(1);
    // let rr = hsem.one_step_lock(2);
    // let tt = hsem.is_semaphore_locked(2);

    mcu_common::hsem::init_hsem_driver(hsem);
    critical_section::set_impl!(mcu_common::hsem::HsemCriticalSection);

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

    let mut led1 = Output::new(p.PB14, Level::High, Speed::Low);
    let mut led2 = Output::new(p.PE1, Level::High, Speed::Low);
    let mut led3 = Output::new(p.PB0, Level::High, Speed::Low);

    loop {
        info!("High");
        led1.set_high();
        cortex_m::asm::delay(1_000_00000);
        led2.set_high();
        cortex_m::asm::delay(1_000_00000);
        led3.set_high();
        cortex_m::asm::delay(1_000_00000);
        info!("Low");
        led1.set_low();
        cortex_m::asm::delay(1_000_00000);
        led2.set_low();
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
