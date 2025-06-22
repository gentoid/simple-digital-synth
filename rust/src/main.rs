#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use stm32f3xx_hal::{dac::Dac, gpio, interrupt, pac, prelude::*};

pub mod consts;
pub mod encoder;
pub mod filter;
pub mod midi;
pub mod oscillator;
pub mod state;
pub mod tables;

use crate::{encoder::Rotation, midi::midi_note_to_freq, oscillator::Oscillator, state::State};

// for Encoder

static CLK_PIN: Mutex<RefCell<Option<gpio::PB0<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static DT_PIN: Mutex<RefCell<Option<gpio::PB1<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static BTN_PIN: Mutex<RefCell<Option<gpio::PB4<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static DAC_HANDLE: Mutex<RefCell<Option<Dac>>> = Mutex::new(RefCell::new(None));
static OSCILLATOR: Mutex<RefCell<Option<Oscillator>>> = Mutex::new(RefCell::new(None));
// static TIM7_HANDLE: Mutex<RefCell<Option<Timer<pac::TIM7>>>> = Mutex::new(RefCell::new(None));

static ENCODER: Mutex<RefCell<crate::encoder::Encoder>> =
    Mutex::new(RefCell::new(crate::encoder::Encoder::new()));

static STATE: Mutex<RefCell<State>> = Mutex::new(RefCell::new(State::new()));

#[entry]
fn main() -> ! {
    info!("Starting the app");
    let mut dp = pac::Peripherals::take().unwrap();

    let rcc_regs = dp.RCC;

    rcc_regs.apb1enr.modify(|_, w| w.tim7en().set_bit());

    let tim7 = &dp.TIM7;
    tim7.cr1.modify(|_, w| {
        w.cen().clear_bit();
        w.udis().clear_bit()
    });
    // tim7.psc.write(|w| w.psc().bits(0));
    // tim7.arr.write(|w| w.arr().bits(332));
    tim7.egr.write(|w| w.ug().set_bit());

    let mut rcc = rcc_regs.constrain();

    let mut flash = dp.FLASH.constrain();
    let _clocks = rcc.cfgr.sysclk(64.MHz()).freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let _pa4 = gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    // let mut timer = Timer::new(dp.TIM7, clocks, &mut rcc.apb1);
    // timer.configure_interrupt(stm32f3xx_hal::timer::Event::Update, true);
    // timer.enable_interrupt(stm32f3xx_hal::timer::Event::Update);
    // timer.start(Nanoseconds(1_000_000_000u32 / SAMPLE_RATE as u32));

    // // Encoder

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let mut clk = gpiob
        .pb0
        .into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let dt = gpiob
        .pb1
        .into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let mut btn = gpiob
        .pb4
        .into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);

    let mut syscfg = dp.SYSCFG.constrain(&mut rcc.apb2);

    syscfg.select_exti_interrupt_source(&clk);
    clk.enable_interrupt(&mut dp.EXTI);
    clk.trigger_on_edge(&mut dp.EXTI, gpio::Edge::Falling);

    syscfg.select_exti_interrupt_source(&btn);
    btn.enable_interrupt(&mut dp.EXTI);
    btn.trigger_on_edge(&mut dp.EXTI, gpio::Edge::Falling);

    let dac = Dac::new(dp.DAC1, &mut rcc.apb1);

    cortex_m::interrupt::free(|cs| {
        let freq = midi_note_to_freq(STATE.borrow(cs).borrow().midi_note);
        let osc = Oscillator::new(freq);

        CLK_PIN.borrow(cs).replace(Some(clk));
        DT_PIN.borrow(cs).replace(Some(dt));
        DAC_HANDLE.borrow(cs).replace(Some(dac));
        OSCILLATOR.borrow(cs).replace(Some(osc));
        BTN_PIN.borrow(cs).replace(Some(btn));
        // TIM7_HANDLE.borrow(cs).replace(Some(timer));
    });

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI0);

        pac::NVIC::unpend(pac::Interrupt::EXTI4);
        pac::NVIC::unmask(pac::Interrupt::EXTI4);

        pac::NVIC::unpend(pac::Interrupt::TIM7);
        pac::NVIC::unmask(pac::Interrupt::TIM7);
    }

    tim7.dier.modify(|_, w| w.uie().set_bit());
    tim7.cr1.modify(|_, w| w.cen().set_bit());

    loop {
        cortex_m::asm::wfi();
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
            let rotation = if dir { Rotation::Left } else { Rotation::Right };
            // let new_note = update_note(direction);

            let mut state = STATE.borrow(cs).borrow_mut();
            let encoder = ENCODER.borrow(cs).borrow();

            state.adjust(&encoder.parameter, rotation);

            // let mut osc = OSCILLATOR.borrow(cs).borrow_mut();
            // if let (Some(osc), Some(note)) = (osc.as_mut(), new_note) {
            //     osc.set_freq(midi_note_to_freq(note));
            // }
        }
    });
}

#[interrupt]
fn EXTI4() {
    cortex_m::interrupt::free(|cs| {
        let mut btn_pin = BTN_PIN.borrow(cs).borrow_mut();

        if let Some(btn) = btn_pin.as_mut() {
            btn.clear_interrupt();

            ENCODER.borrow(cs).borrow_mut().next_param();
        }
    });
}

#[interrupt]
fn TIM7() {
    cortex_m::interrupt::free(|cs| {
        // let mut tim7 = TIM7_HANDLE.borrow(cs).borrow_mut();
        // if let Some(tim) = tim7.as_mut() {
        //     tim.clear_event(stm32f3xx_hal::timer::Event::Update);
        // }

        let mut dac = DAC_HANDLE.borrow(cs).borrow_mut();
        let mut osc = OSCILLATOR.borrow(cs).borrow_mut();

        if let (Some(dac), Some(osc)) = (dac.as_mut(), osc.as_mut()) {
            let sample = osc.next_sample();
            dac.write_data(sample);
        }
    });
}
