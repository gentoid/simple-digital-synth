#![no_main]
#![no_std]

use core::{
    cell::RefCell,
    sync::atomic::{AtomicI16, Ordering},
};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{
    self as hal,
    delay::Delay,
    gpio::{Edge, Input, PA0, PA1},
    interrupt,
};

use hal::{pac, prelude::*};
use stm32f3xx_hal::pwm;

// for Encoder

static CLK_PIN: Mutex<RefCell<Option<PA0<Input>>>> = Mutex::new(RefCell::new(None));
static DT_PIN: Mutex<RefCell<Option<PA1<Input>>>> = Mutex::new(RefCell::new(None));
static COUNT: AtomicI16 = AtomicI16::new(1);

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(40.MHz()).freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut delay = Delay::new(cp.SYST, clocks);

    // sawtooth

    let pa8 = gpioa
        .pa8
        .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let tim1_channels = pwm::tim1(dp.TIM1, 256 - 1, 30_000.Hz(), &clocks);

    let mut tim1_ch1 = tim1_channels.0.output_to_pa8(pa8);
    tim1_ch1.enable();

    let max_duty = tim1_ch1.get_max_duty();
    let mut duty = 0;

    // Encoder

    let mut clk = gpioa
        .pa0
        .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
    let dt = gpioa
        .pa1
        .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

    clk.enable_interrupt(&mut dp.EXTI);
    clk.trigger_on_edge(&mut dp.EXTI, Edge::Falling);

    cortex_m::interrupt::free(|cs| {
        CLK_PIN.borrow(cs).replace(Some(clk));
        DT_PIN.borrow(cs).replace(Some(dt));
    });

    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::interrupt::EXTI0);
    }

    loop {
        let count = COUNT.load(Ordering::Relaxed) as u16;
        tim1_ch1.set_duty(duty);

        duty += 7 + count / 5;
        if duty >= max_duty {
            duty = 0;
        }

        delay.delay_us(count * count);
    }
}

#[interrupt]
fn EXTI0() {
    cortex_m::interrupt::free(|cs| {
        let mut clk_pin = CLK_PIN.borrow(cs).borrow_mut();
        let dt_pin = DT_PIN.borrow(cs).borrow();

        if let (Some(clk), Some(dt)) = (clk_pin.as_mut(), dt_pin.as_ref()) {
            clk.clear_interrupt();

            let dir = dt.is_high().unwrap_or(false);
            let diff = if dir { 1 } else { -1 };
            update_count(diff);
        }
    });
}

fn update_count(diff: i16) {
    COUNT
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            let new = current.saturating_add(diff).clamp(1, 30);
            Some(new)
        })
        .ok();
}
