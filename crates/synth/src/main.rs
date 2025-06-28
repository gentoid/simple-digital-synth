#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use midi_parser::parser::{MidiChannel, MidiMessageKind, RunningStatus};
use panic_probe as _;
use stm32f3xx_hal::{
    dac::Dac,
    gpio::{self, AF7, PA9, PA10, PushPull},
    interrupt, nb,
    pac::{self, USART1},
    prelude::*,
    serial::{self, Event, Serial, config::Config},
};

pub mod consts;
pub mod encoder;
pub mod filter;
pub mod oscillator;
pub mod state;

use crate::{consts::MAX_DAC_VALUE, encoder::Rotation, state::State};

// for Encoder

static CLK_PIN: Mutex<RefCell<Option<gpio::PB0<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static DT_PIN: Mutex<RefCell<Option<gpio::PB1<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static BTN_PIN: Mutex<RefCell<Option<gpio::PB4<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static DAC_HANDLE: Mutex<RefCell<Option<Dac>>> = Mutex::new(RefCell::new(None));
// static TIM7_HANDLE: Mutex<RefCell<Option<Timer<pac::TIM7>>>> = Mutex::new(RefCell::new(None));

// MIDI

static UART: Mutex<RefCell<Option<Serial<USART1, (PA9<AF7<PushPull>>, PA10<AF7<PushPull>>)>>>> =
    Mutex::new(RefCell::new(None));

static MIDI: Mutex<RefCell<Option<RunningStatus>>> = Mutex::new(RefCell::new(None));

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
    let clocks = rcc.cfgr.sysclk(64.MHz()).freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let _pa4 = gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    // MINI input

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

    let parser = RunningStatus::new(MidiChannel::Ch1);

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
        CLK_PIN.borrow(cs).replace(Some(clk));
        DT_PIN.borrow(cs).replace(Some(dt));
        DAC_HANDLE.borrow(cs).replace(Some(dac));
        BTN_PIN.borrow(cs).replace(Some(btn));
        // TIM7_HANDLE.borrow(cs).replace(Some(timer));

        // MIDI
        UART.borrow(cs).replace(Some(serial));
        MIDI.borrow(cs).replace(Some(parser));
    });

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI0);

        pac::NVIC::unpend(pac::Interrupt::EXTI4);
        pac::NVIC::unmask(pac::Interrupt::EXTI4);

        pac::NVIC::unpend(pac::Interrupt::TIM7);
        pac::NVIC::unmask(pac::Interrupt::TIM7);

        // MIDI
        pac::NVIC::unmask(pac::Interrupt::USART1_EXTI25);
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

#[interrupt]
fn TIM7() {
    cortex_m::interrupt::free(|cs| {
        // let mut tim7 = TIM7_HANDLE.borrow(cs).borrow_mut();
        // if let Some(tim) = tim7.as_mut() {
        //     tim.clear_event(stm32f3xx_hal::timer::Event::Update);
        // }

        let mut dac = DAC_HANDLE.borrow(cs).borrow_mut();
        let mut state = STATE.borrow(cs).borrow_mut();

        if let Some(dac) = dac.as_mut() {
            let sample = state.oscillator.next_sample();
            let filtered = state.filter.process(sample);
            let as_u16 = (filtered as u16).clamp(0, MAX_DAC_VALUE);
            dac.write_data(as_u16);
        }
    });
}

// MIDI
#[interrupt]
fn USART1_EXTI25() {
    cortex_m::interrupt::free(|cs| {
        if let (Some(uart), Some(midi)) = (
            UART.borrow(cs).borrow_mut().as_mut(),
            MIDI.borrow(cs).borrow_mut().as_mut(),
        ) {
            match uart.read() {
                Ok(byte) => {
                    debug!(" == Byte: 0x{:02X} | 0b{:08b}", byte, byte);
                    midi.process_midi_byte(byte);

                    if midi.message_kind().is_none() {
                        return;
                    }

                    if midi.in_progress() {
                        return;
                    }

                    use MidiMessageKind::*;

                    match midi.message_kind().as_ref().unwrap() {
                        NoteOn(note, velocity) if velocity.0 > 0 => {
                            info!("Start note: {} with velocity: {}", note.0, velocity.0);
                        }
                        NoteOff(note, velocity) | NoteOn(note, velocity) => {
                            info!("Stop note: {} with velocity: {}", note.0, velocity.0)
                        }
                        _ => {}
                    }
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
