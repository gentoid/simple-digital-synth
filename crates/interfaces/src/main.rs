#![no_main]
#![no_std]

// use core::mem::MaybeUninit;

use defmt::info;
// use defmt::{debug, info};
// // use embassy_executor::Spawner;
use embassy_stm32::{
//     Config, SharedData, bind_interrupts, exti,
    gpio::{self, Level, Output, Speed},
//     hsem::HardwareSemaphore,
//     peripherals::{self, DMA2_CH2, PA0, PA1, PA2, PA10},
//     usart,
};
// // use embassy_time::{Duration, Timer};
// use midi_parser::parser::{MidiChannel, RunningStatus};
use {defmt_rtt as _, panic_probe as _};
// use panic_probe as _;

// use crate::encoder::Rotation;

// pub mod encoder;

// #[unsafe(link_section = ".ram_d3.shared_data")]
// static SHARED_DATA: MaybeUninit<SharedData> = MaybeUninit::uninit();

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = unsafe { embassy_stm32::Peripherals::steal() };
    let mut led2 = Output::new(dp.PE1, Level::High, Speed::Low);

    loop {
        info!("M4: High");
        // led1.set_high();
        cortex_m::asm::delay(4_172_500);
        led2.set_high();
        cortex_m::asm::delay(4_172_500);
        // led3.set_high();
        cortex_m::asm::delay(4_172_500);
        info!("M4: Low");
        // led1.set_low();
        cortex_m::asm::delay(4_172_500);
        led2.set_low();
        cortex_m::asm::delay(4_172_500);
        // led3.set_low();
        cortex_m::asm::delay(4_172_500);
    }
    // let p = embassy_stm32::init_secondary(&SHARED_DATA);

    // let p = unsafe { embassy_stm32::Peripherals::steal() };
    // let hsem = HardwareSemaphore::new(p.HSEM);

    // mcu_common::hsem::init_hsem_driver(hsem);
    // critical_section::set_impl!(mcu_common::hsem::HsemCriticalSection);
    
    // info!("M4: HSEM has been successfully set up");
    

    // info!("M4: Starting the M4 core");

    // let mut led = Output::new(p.PE1, Level::High, Speed::Low);

    // loop {
    //     info!("M4: High");
    //     led.set_high();
    //     cortex_m::asm::delay(16_000_00000);
    //     info!("M4: Low");
    //     led.set_low();
    //     cortex_m::asm::delay(16_000_00000);
    // }
}

// bind_interrupts!(struct Irqs {
//     USART1 => usart::InterruptHandler<peripherals::USART1>;
// });

// #[embassy_executor::main]
// async fn main(spawner: Spawner) {
//     // info!("Starting the M4 core");

//     critical_section::set_impl!(mcu_common::hsem::HsemCriticalSection);
//     let p = embassy_stm32::init_secondary(&SHARED_DATA);
//     let hsem = HardwareSemaphore::new(p.HSEM);
//     mcu_common::hsem::init_hsem_driver(hsem);

//     info!("M4 configured");

//     spawner.spawn(blink_led(p.PE1)).unwrap();
//     spawner
//         .spawn(receive_midi_messages(p.USART1, p.PA10, p.DMA2_CH2))
//         .unwrap();
//     spawner.spawn(button_task(p.PA2, p.EXTI2)).unwrap();
//     spawner.spawn(encoder_task(p.PA0, p.PA1, p.EXTI0)).unwrap();

// }

// #[embassy_executor::task]
// async fn blink_led(pin: peripherals::PE1) {
//     let mut led = Output::new(pin, Level::High, Speed::Low);

//     loop {
//         info!("High");
//         led.set_high();
//         cortex_m::asm::delay(4_172_50000);
//         info!("Low");
//         led.set_low();
//         cortex_m::asm::delay(4_172_50000);
//     }
// }

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
