#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use dsp as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = stm32h7xx_hal::pac, dispatchers = [EXTI1], peripherals = true)]
mod app {
    use defmt::warn;
    use dsp::state::State;
    use midi_parser::parser::{MidiChannel, RunningStatus};
    use rtic_sync::{
        channel::{self, NoReceiver},
        make_channel,
    };
    use stm32h7xx_hal::{
        nb, pac,
        prelude::*,
        serial,
        timer::{Event, Timer},
    };

    const MIDI_IN_MSG_CAPACITY: usize = 8;

    type MidiRxSender = channel::Sender<'static, u8, MIDI_IN_MSG_CAPACITY>;
    type MidiRxReceiver = channel::Receiver<'static, u8, MIDI_IN_MSG_CAPACITY>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        state: State,
        sample_timer: pac::TIM2,
        midi_rx: serial::Rx<pac::USART3>,
        midi_parser: RunningStatus,
        midi_rx_send: MidiRxSender,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let dp = cx.device;

        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();

        let rcc = dp.RCC.constrain();
        let ccdr = rcc.sys_ck(400u32.MHz()).freeze(pwrcfg, &dp.SYSCFG);

        // Sample timer
        let mut timer = Timer::tim2(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);

        timer.start(48u32.kHz());
        timer.listen(Event::TimeOut);

        let (sample_timer, _) = timer.free();

        // MIDI
        let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
        let _tx = gpiod.pd8.into_alternate::<7>();
        let _rx = gpiod.pd9.into_alternate::<7>();

        let serial = serial::Serial::usart3(
            dp.USART3,
            serial::config::Config::default().baudrate(31_250.bps()),
            ccdr.peripheral.USART3,
            &ccdr.clocks,
            false,
        )
        .unwrap();

        let (_, mut rx) = serial.split();
        rx.listen();

        // MIDI IN channel
        let (midi_rx_send, midi_rx_recv) = make_channel!(u8, MIDI_IN_MSG_CAPACITY);

        // Spawn tasks
        process_midi_bytes::spawn(midi_rx_recv).unwrap();

        (
            Shared {},
            Local {
                state: State::new(),
                sample_timer,
                midi_rx: rx,
                midi_parser: RunningStatus::new(MidiChannel::Ch1),
                midi_rx_send,
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

    #[task(binds = TIM2, priority = 9, local = [sample_timer, state])]
    fn generator(cx: generator::Context) {
        let tim = cx.local.sample_timer;
        tim.sr.modify(|_, w| w.uif().clear_bit());

        cx.local.state.next_sample();
    }

    #[task(binds = USART3, priority = 8, local = [midi_rx, midi_rx_send])]
    fn midi_rx(cx: midi_rx::Context) {
        match cx.local.midi_rx.read() {
            Ok(byte) => {
                match cx.local.midi_rx_send.try_send(byte) {
                    Ok(()) => {}
                    Err(channel::TrySendError::NoReceiver(_)) => {
                        // todo handle the case
                    }
                    Err(channel::TrySendError::Full(_)) => {
                        // todo handle the case
                    }
                };
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

    #[task(priority = 7, local = [midi_parser])]
    async fn process_midi_bytes(cx: process_midi_bytes::Context, mut recv: MidiRxReceiver) {
        while let Ok(byte) = recv.recv().await {
            cx.local.midi_parser.process_midi_byte(byte);
            // todo what next?
        }

        // todo what if Err?
    }
}
