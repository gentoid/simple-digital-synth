#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use stm32f3xx_hal::{
    dac::Dac,
    gpio::{self, AF7, PA9, PA10, PushPull},
    interrupt, nb,
    pac::{self, USART1},
    prelude::*,
    rcc,
    serial::{self, Event, Serial, config::Config},
};

pub mod adsr;
pub mod consts;
pub mod encoder;
pub mod filter;
pub mod oscillator;
pub mod state;
pub mod voice;

use crate::{
    consts::{MAX_DAC_VALUE, SAMPLE_RATE},
    encoder::Rotation,
    state::State,
};

// for Encoder

static CLK_PIN: Mutex<RefCell<Option<gpio::PB0<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static DT_PIN: Mutex<RefCell<Option<gpio::PB1<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static BTN_PIN: Mutex<RefCell<Option<gpio::PB4<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static DAC_HANDLE: Mutex<RefCell<Option<Dac>>> = Mutex::new(RefCell::new(None));

// MIDI

static UART: Mutex<RefCell<Option<Serial<USART1, (PA9<AF7<PushPull>>, PA10<AF7<PushPull>>)>>>> =
    Mutex::new(RefCell::new(None));

static ENCODER: Mutex<RefCell<crate::encoder::Encoder>> =
    Mutex::new(RefCell::new(crate::encoder::Encoder::new()));

static STATE: Mutex<RefCell<State>> = Mutex::new(RefCell::new(State::new()));

#[entry]
fn main() -> ! {
    info!("Starting the app");
    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();

    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(64.MHz()).freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let _pa4 = gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    // MIDI input

    let midi_tx =
        gpioa
            .pa9
            .into_af_push_pull::<7>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let midi_rx =
        gpioa
            .pa10
            .into_af_push_pull::<7>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let mut serial = Serial::new(
        dp.USART1,
        (midi_tx, midi_rx),
        Config::default().baudrate(31_250.Bd()),
        clocks,
        &mut rcc.apb2,
    );
    serial.configure_interrupt(Event::ReceiveDataRegisterNotEmpty, true);

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
        CLK_PIN.borrow(cs).replace(Some(clk));
        DT_PIN.borrow(cs).replace(Some(dt));
        DAC_HANDLE.borrow(cs).replace(Some(dac));
        BTN_PIN.borrow(cs).replace(Some(btn));

        // MIDI
        UART.borrow(cs).replace(Some(serial));
    });

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI0);

        pac::NVIC::unpend(pac::Interrupt::EXTI4);
        pac::NVIC::unmask(pac::Interrupt::EXTI4);

        // MIDI
        pac::NVIC::unmask(pac::Interrupt::USART1_EXTI25);
    }

    setup_tim7(&dp.TIM7, &clocks);

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

            let mut state = STATE.borrow(cs).borrow_mut();
            let encoder = ENCODER.borrow(cs).borrow();

            state.adjust(&encoder.parameter, rotation);
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

fn setup_tim7(tim: &pac::TIM7, clocks: &rcc::Clocks) {
    // Enable tim7
    let rcc = unsafe { &*pac::RCC::ptr() };
    rcc.apb1enr.modify(|_, w| w.tim7en().set_bit());

    let pclk = clocks.pclk1().0;

    let target_freq = SAMPLE_RATE as u32;
    let mut psc = 0;
    let mut arr = pclk / target_freq;

    while arr > 65535 {
        psc += 1;
        arr = pclk / (target_freq * (psc + 1));
    }

    // Turn it off before setting up
    tim.cr1.modify(|_, w| w.cen().clear_bit());

    tim.psc.write(|w| w.psc().bits(0));
    tim.arr.write(|w| w.arr().bits((arr - 1) as u16));

    tim.egr.write(|w| w.ug().set_bit());

    tim.dier.write(|w| w.uie().set_bit());
    tim.cr1.write(|w| w.cen().set_bit());

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIM7);
    }
}

#[interrupt]
fn TIM7() {
    let dp = unsafe { pac::Peripherals::steal() };
    let tim = &dp.TIM7;
    tim.sr.modify(|_, w| w.uif().clear_bit());

    cortex_m::interrupt::free(|cs| {
        let mut dac = DAC_HANDLE.borrow(cs).borrow_mut();
        let mut state = STATE.borrow(cs).borrow_mut();

        if let Some(dac) = dac.as_mut() {
            let sample = state.next_sample();
            let filtered = state.filter.process(sample);
            let as_u16 =
                (((filtered + 1.0) * MAX_DAC_VALUE as f32 / 2.0) as u16).clamp(0, MAX_DAC_VALUE);
            dac.write_data(as_u16);
        }
    });
}

// MIDI
#[interrupt]
fn USART1_EXTI25() {
    cortex_m::interrupt::free(|cs| {
        if let Some(uart) = UART.borrow(cs).borrow_mut().as_mut() {
            match uart.read() {
                Ok(byte) => {
                    let mut state = STATE.borrow(cs).borrow_mut();
                    // debug!(" == Byte: 0x{:02X} | 0b{:08b}", byte, byte);
                    state.process_midi_byte(byte);
                }
                Err(err) => match err {
                    nb::Error::Other(err) => match err {
                        serial::Error::Framing => warn!("Error: Framing"),
                        serial::Error::Noise => warn!("Error: Noise"),
                        serial::Error::Overrun => warn!("Error: Overrun"),
                        serial::Error::Parity => warn!("Error: Parity"),
                        _ => warn!("Error: unknown"),
                    },
                    nb::Error::WouldBlock => warn!("Would block"),
                },
            }
        }
    });
}
