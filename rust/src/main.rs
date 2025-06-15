#![no_std]
#![no_main]

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use panic_halt as _;

use stm32f3xx_hal::{delay::Delay, pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut led = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.toggle().unwrap();
        delay.delay_ms(500_u16);
    }
}
