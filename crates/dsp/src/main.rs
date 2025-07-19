#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use dsp as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = stm32h7xx_hal::pac, dispatchers = [], peripherals = true)]
mod app {
    use dsp::state::State;
    use stm32h7xx_hal::{
        pac,
        prelude::*,
        timer::{Event, Timer},
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        state: State,
        sample_timer: pac::TIM2,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let dp = cx.device;

        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();

        let rcc = dp.RCC.constrain();
        let ccdr = rcc.sys_ck(400u32.MHz()).freeze(pwrcfg, &dp.SYSCFG);

        let mut timer = Timer::tim2(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);

        timer.start(48u32.kHz());
        timer.listen(Event::TimeOut);

        let (sample_timer, _) = timer.free();

        (
            Shared {},
            Local {
                state: State::new(),
                sample_timer,
            },
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }
}
