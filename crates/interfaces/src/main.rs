#![no_main]
#![no_std]

use core::{fmt::Debug, mem::MaybeUninit};

// use defmt::{debug, error, info};
use embassy_executor::Spawner;
use embassy_stm32::{
    Config, SharedData, bind_interrupts, exti,
    gpio::{self, Level, Output, Speed},
    hsem::HardwareSemaphore,
    peripherals::{self, DMA2_CH2, PA0, PA1, PA2, PA10},
    usart,
};
use embassy_time::Timer;
// use embassy_time::{Duration, Timer};
use midi_parser::parser::{MidiChannel, RunningStatus};
// use {defmt_rtt as _, panic_probe as _};
use panic_halt as _;

// use crate::encoder::Rotation;

// pub mod encoder;

#[unsafe(link_section = ".ram_d3.shared_data")]
static SHARED_DATA: MaybeUninit<SharedData> = MaybeUninit::uninit();

// bind_interrupts!(struct Irqs {
//     USART1 => usart::InterruptHandler<peripherals::USART1>;
// });

// #[cortex_m_rt::exception]
// unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
//     defmt::dbg!("{:#?}", ef);
//     error!("HardFault");
//     loop {}
// }

#[cortex_m_rt::pre_init]
unsafe fn fix_vtor() {
    let scb = unsafe { &*cortex_m::peripheral::SCB::PTR };
    unsafe { scb.vtor.write(0x0810_0000) };

    let p = unsafe { embassy_stm32::Peripherals::steal() };
    let hsem = HardwareSemaphore::new(p.HSEM);

    mcu_common::hsem::init_hsem_driver(hsem);
    critical_section::set_impl!(mcu_common::hsem::HsemCriticalSection);

    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    unsafe { invalidate_dcache() };

    let p = embassy_stm32::init_secondary(&SHARED_DATA);
    
    let pin = p.PE1;
    let mut led2 = Output::new(pin, Level::High, Speed::Low);

    // loop {
        // info!("M4: High");
        cortex_m::asm::delay(1_170_0000);
        led2.set_high();
        cortex_m::asm::delay(1_170_0000);
        cortex_m::asm::delay(1_170_0000);
        // // info!("M4: Low");
        cortex_m::asm::delay(1_170_0000);
        led2.set_low();
        cortex_m::asm::delay(1_170_0000);
        cortex_m::asm::delay(1_170_0000);
    // }
}

unsafe fn invalidate_dcache() {
    const SCB_BASE: u32 = 0xE000ED00;
    const SCB_CSSELR: *mut u32 = (SCB_BASE + 0x80 + 0x00) as *mut u32;
    const SCB_CCSIDR: *const u32 = (SCB_BASE + 0x80 + 0x04) as *const u32;
    const SCB_DCISW: *mut u32 = (SCB_BASE + 0x80 + 0x0C) as *mut u32;

    // D-cache Level 1
    unsafe { SCB_CSSELR.write_volatile(0) };
    cortex_m::asm::dsb();

    let ccsidr = unsafe { SCB_CCSIDR.read_volatile() };
    let num_sets = ((ccsidr >> 13) & 0x7FFF) + 1;
    let num_ways = ((ccsidr >> 3) & 0x3FF) + 1;

    for set in 0..num_sets {
        for way in 0..num_ways {
            let set_way = (way << 30) | (set << 5);
            unsafe { SCB_DCISW.write_volatile(set_way) };
        }
    }

    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // info!("M4: HSEM has been successfully set up");

    let p = unsafe { embassy_stm32::Peripherals::steal() };
    spawner.spawn(blink_led(p.PE1)).unwrap();
    // spawner
    //     .spawn(receive_midi_messages(p.USART1, p.PA10, p.DMA2_CH2))
    //     .unwrap();
    // spawner.spawn(button_task(p.PA2, p.EXTI2)).unwrap();
    // spawner.spawn(encoder_task(p.PA0, p.PA1, p.EXTI0)).unwrap();
}

#[embassy_executor::task]
async fn blink_led(pin: peripherals::PE1) {
    let mut led2 = Output::new(pin, Level::High, Speed::Low);

    loop {
        // info!("M4: High");
        Timer::after_millis(500).await;
        // cortex_m::asm::delay(1_170_00000);
        led2.set_high();
        Timer::after_millis(500).await;
        Timer::after_millis(500).await;
        Timer::after_millis(500).await;
        // cortex_m::asm::delay(1_170_00000);
        // cortex_m::asm::delay(1_170_00000);
        // // info!("M4: Low");
        // cortex_m::asm::delay(1_170_00000);
        led2.set_low();
        Timer::after_millis(500).await;
        Timer::after_millis(500).await;
        // cortex_m::asm::delay(1_170_00000);
        // cortex_m::asm::delay(1_170_00000);
    }
}

// #[embassy_executor::task]
// async fn receive_midi_messages(usart: peripherals::USART1, pin: PA10, ch: DMA2_CH2) {
//     let mut config = usart::Config::default();
//     config.baudrate = 31_250;

//     let mut uart = usart::UartRx::new(usart, Irqs, pin, ch, config).unwrap();

//     let mut buf = [0u8; 1];
//     let mut parser = RunningStatus::new(MidiChannel::Ch1);

//     loop {
//         if let Ok(_) = uart.read(&mut buf).await {
//             debug!("Received byte: {}", buf);
//             parser.process_midi_byte(buf[0]);
//         }
//     }
// }

// #[embassy_executor::task]
// async fn button_task(pin: PA2, ch: peripherals::EXTI2) {
//     let mut exti = exti::ExtiInput::new(pin, ch, gpio::Pull::Up);

//     loop {
//         exti.wait_for_falling_edge().await; // Pressed
//         Timer::after(Duration::from_millis(10)).await; // debounce
//         if exti.is_low() {
//             info!("Pressed on PA2");
//         }

//         exti.wait_for_rising_edge().await; // Released
//         Timer::after(Duration::from_millis(10)).await; // debounce
//         if exti.is_high() {
//             info!("Released on PA2");
//         }
//     }
// }

// #[embassy_executor::task]
// async fn encoder_task(pin_a: PA0, pin_b: PA1, ch: peripherals::EXTI0) {
//     let mut exti = exti::ExtiInput::new(pin_a, ch, gpio::Pull::Up);
//     let phase_b = gpio::Input::new(pin_b, gpio::Pull::Up);

//     loop {
//         exti.wait_for_rising_edge().await;
//         let dir = if phase_b.get_level() == gpio::Level::High {
//             Rotation::Right;
//         } else {
//             Rotation::Left;
//         };

//         info!("Rotation on PA0/PA1 detected: {}", dir);
//     }
// }
