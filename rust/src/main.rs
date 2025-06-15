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

    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let pa7 = gpioa.pa7.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let tim3_channels = pwm::tim3(dp.TIM3, 1280, 5000.Hz(),  &clocks);

    let mut tim3_ch2 = tim3_channels.1.output_to_pa7(pa7);
    tim3_ch2.enable();

    let max_duty = tim3_ch2.get_max_duty();
    let mut duty = 0;

    loop {
        tim3_ch2.set_duty(duty);
        duty += 1;
        if duty >= max_duty {
            duty = 0;
        }
    }
}
