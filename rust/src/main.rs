#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal as hal;

use hal::{
    prelude::*,
    pac,
};
use stm32f3xx_hal::pwm;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(67.MHz()).freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let pa8 = gpioa.pa8.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let tim1_channels = pwm::tim1(dp.TIM1, 256 - 1, 300_000.Hz(), &clocks);

    let mut tim1_ch1 = tim1_channels.0.output_to_pa8(pa8);
    tim1_ch1.enable();

    let max_duty = tim1_ch1.get_max_duty();
    let mut duty = 0;

    loop {
        tim1_ch1.set_duty(duty);
        duty += 1;
        if duty >= max_duty {
            duty = 0;
        }
    }
}
