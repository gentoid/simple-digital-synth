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
    pac::RCC.ahb4enr().modify(|w| {
        w.set_hsemen(true);
        w.set_bkpsramen(true);
    });

    pac::RCC.ahb2enr().modify(|w| {
        w.set_sram1en(true);
        w.set_sram2en(true);
        w.set_sram3en(true);
    });

    pac::PWR.cr3().modify(|w| w.ldoen());
    while !pac::PWR.csr1().read().actvosrdy() {}
    
    pac::PWR.cpucr().modify(|w| w.set_pdds_d2(false));

    // pac::RCC.apb2rstr().modify(|w| w.);

    pac::EXTI.emr(0).modify(|w| w.);

    pac::RCC.ahb1enr().modify(|w| w.set_dma1en(true));

    pac::RCC.ahb3enr().modify(|w| w.set_axisramen(true));
    pac::RCC.c1_ahb4enr().modify(|w| w.set_gpioeen(true));

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
    let r = unsafe { SHARED_DATA.assume_init_ref() };
    let init_flag = r.init_flag.load(core::sync::atomic::Ordering::SeqCst);
    info!("init_flag {}", init_flag);
    // info!("SH D: {}", r)

    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    unsafe { flush_dcache() };

    pac::RCC.gcr().modify(|w| w.set_boot_c2(true));

    let mc4_booted = pac::RCC.gcr().modify(|w| w.boot_c2());
    info!("M7: CM4 boot enabled: {}", mc4_booted);

    info!("M7: Embassy STM32 initialized!");

    let mut led1 = Output::new(p.PB14, Level::High, Speed::Low);
    // let mut led2 = Output::new(p.PE1, Level::High, Speed::Low);
    let mut led3 = Output::new(p.PB0, Level::High, Speed::Low);

    loop {
        // cortex_m::asm::nop();
        // info!("High");
        led1.set_high();
        cortex_m::asm::delay(10_000_000);
        // led2.set_high();
        cortex_m::asm::delay(10_000_000);
        led3.set_high();
        cortex_m::asm::delay(10_000_000);
        // info!("Low");
        led1.set_low();
        cortex_m::asm::delay(10_000_000);
        // led2.set_low();
        cortex_m::asm::delay(10_000_000);
        led3.set_low();
        cortex_m::asm::delay(10_000_000);
    }
}

unsafe fn flush_dcache() {
    const SCB_BASE: u32 = 0xE000ED00;
    const SCB_CCSIDR: *const u32 = (SCB_BASE + 0x80 + 0x04) as *const u32;
    const SCB_CSSELR: *mut u32 = (SCB_BASE + 0x80 + 0x00) as *mut u32;
    const SCB_DCCISW: *mut u32 = (SCB_BASE + 0x80 + 0x0C) as *mut u32;

    unsafe { SCB_CSSELR.write_volatile(0) }; // Select Level 1 data cache
    cortex_m::asm::dsb();

    let ccsidr = unsafe { SCB_CCSIDR.read_volatile() };
    let sets = ((ccsidr >> 13) & 0x7fff) + 1;
    let ways = ((ccsidr >> 3) & 0x3ff) + 1;

    for set in 0..sets {
        for way in 0..ways {
            let sw = (way << 30) | (set << 5);
            unsafe { SCB_DCCISW.write_volatile(sw) };
        }
    }

    cortex_m::asm::dsb();
    cortex_m::asm::isb();
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
