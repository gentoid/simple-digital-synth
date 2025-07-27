#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use dsp as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = stm32h7xx_hal::pac, dispatchers = [EXTI1], peripherals = true)]
mod app {
    use dsp::{lcd::HD44780, midi::MidiRxReceiver, state::State};
    use midi_parser::parser::{MidiChannel, MidiParser};
    use rtic_sync::{channel::ReceiveError, make_channel};
    use stm32h7xx_hal::{
        i2c, pac,
        prelude::*,
        serial,
        timer::{Event, Timer},
    };

    #[shared]
    struct Shared {
        state: State,
    }

    #[local]
    struct Local {
        sample_timer: pac::TIM2,
        midi_rx: serial::Rx<pac::USART3>,
        midi_parser: MidiParser,
        midi_rx_send: dsp::midi::MidiRxSender,
        lcd: HD44780<i2c::I2c<pac::I2C1>, cortex_m::delay::Delay>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let dp = cx.device;

        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();

        let rcc = dp.RCC.constrain();
        let ccdr = rcc.sys_ck(400u32.MHz()).freeze(pwrcfg, &dp.SYSCFG);

        let delay = cortex_m::delay::Delay::new(cx.core.SYST, ccdr.clocks.sys_ck().raw());

        let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
        let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

        // Sample timer
        let mut timer = Timer::tim2(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);

        timer.start(48u32.kHz());
        timer.listen(Event::TimeOut);

        let (sample_timer, _) = timer.free();

        // MIDI
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
        let (midi_rx_send, midi_rx_recv) = make_channel!(u8, { dsp::midi::MIDI_RX_CAPACITY });

        // LCD
        let scl = gpiob.pb8.into_alternate().set_open_drain();
        let sda = gpiob.pb9.into_alternate().set_open_drain();

        let i2c = dp
            .I2C1
            .i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);

        let lcd = HD44780::new(i2c, delay, 0x3F);

        // Spawn tasks
        process_midi_bytes::spawn(midi_rx_recv).unwrap();
        lcd_task::spawn().unwrap();

        (
            Shared {
                state: State::new(),
            },
            Local {
                sample_timer,
                midi_rx: rx,
                midi_parser: MidiParser::new(MidiChannel::Ch1),
                midi_rx_send,
                lcd,
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

    #[task(binds = TIM2, priority = 9, local = [sample_timer], shared = [state])]
    fn generator(mut cx: generator::Context) {
        let tim = cx.local.sample_timer;
        tim.sr.modify(|_, w| w.uif().clear_bit());

        // todo send over to I2S
        cx.shared.state.lock(|state| state.next_sample());
    }

    #[task(binds = USART3, priority = 8, local = [midi_rx, midi_rx_send])]
    fn midi_rx(cx: midi_rx::Context) {
        dsp::midi::enqueue_midi_processing(cx.local.midi_rx, cx.local.midi_rx_send);
    }

    #[task(priority = 7, local = [midi_parser], shared = [state])]
    async fn process_midi_bytes(mut cx: process_midi_bytes::Context, mut recv: MidiRxReceiver) {
        loop {
            match recv.recv().await {
                Ok(byte) => {
                    if let Some(msg) = cx.local.midi_parser.process(byte) {
                        cx.shared.state.lock(|state| state.process_midi_msg(&msg));
                    }
                }
                Err(ReceiveError::Empty) => {
                    defmt::warn!("MIDI RX: the queue is empty");
                    // todo delay or break?
                }
                Err(ReceiveError::NoSender) => {
                    defmt::warn!("MIDI RX channel closed");
                    break;
                }
            }
        }
    }

    #[task(priority = 7, local = [lcd])]
    async fn lcd_task(ctx: lcd_task::Context) {
        let lcd = ctx.local.lcd;
        lcd.init();

        lcd.set_row(0);
        lcd.write_str("Param:");
        lcd.set_row(1);
        lcd.write_str("123 Hz");
    }
}
