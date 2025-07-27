#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use dsp as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = stm32h7xx_hal::pac, dispatchers = [EXTI1], peripherals = true)]
mod app {
    use defmt::warn;
    use dsp::{
        audio::{AudioState, BUFFER_LEN, SaiTxDma, SampleType},
        midi::MidiRxReceiver,
        state::State,
    };
    use midi_parser::parser::{MidiChannel, MidiParser};
    use rtic_sync::{channel::ReceiveError, make_channel};
    use stm32h7xx_hal::{
        dma,
        // timer::{Event, Timer},
        gpio,
        pac,
        prelude::*,
        sai,
        serial,
    };

    #[shared]
    struct Shared {
        state: State,
        audio_state: AudioState,
    }

    #[local]
    struct Local {
        // sample_timer: pac::TIM2,
        midi_rx: serial::Rx<pac::USART2>,
        midi_parser: MidiParser,
        midi_rx_send: dsp::midi::MidiRxSender,
        // lcd: HD44780<i2c::I2c<pac::I2C1>, cortex_m::delay::Delay>,
        _sai: sai::Sai<stm32h7xx_hal::stm32::SAI1, sai::I2S>,
        transfer: dma::Transfer<
            dma::dma::StreamX<pac::DMA2, 0>,
            SaiTxDma<SampleType>,
            dma::MemoryToPeripheral,
            &'static mut [SampleType; BUFFER_LEN],
            dma::DBTransfer,
        >,
        tmp_buf: &'static mut [SampleType; BUFFER_LEN],
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let dp = cx.device;

        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();

        let rcc = dp.RCC.constrain();
        let ccdr = rcc.sys_ck(400u32.MHz()).freeze(pwrcfg, &dp.SYSCFG);

        // let delay = cortex_m::delay::Delay::new(cx.core.SYST, ccdr.clocks.sys_ck().raw());

        // let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
        let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
        let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

        // Sample timer
        // let mut timer = Timer::tim2(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);

        let sample_rate = 48u32.kHz();
        // timer.start(sample_rate);
        // timer.listen(Event::TimeOut);

        // let (sample_timer, _) = timer.free();

        // MIDI
        let _tx = gpiod.pd5.into_alternate::<7>();
        let _rx = gpiod.pd6.into_alternate::<7>();

        let serial = serial::Serial::usart2(
            dp.USART2,
            serial::config::Config::default().baudrate(31_250.bps()),
            ccdr.peripheral.USART2,
            &ccdr.clocks,
            false,
        )
        .unwrap();

        let (_, mut rx) = serial.split();
        rx.listen();

        // MIDI IN channel
        let (midi_rx_send, midi_rx_recv) = make_channel!(u8, { dsp::midi::MIDI_RX_CAPACITY });

        // LCD
        // let scl = gpiob.pb8.into_alternate().set_open_drain();
        // let sda = gpiob.pb9.into_alternate().set_open_drain();

        // let i2c = dp
        //     .I2C1
        //     .i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);

        // let lcd = HD44780::new(i2c, delay, 0x3F);

        // DMA + I2S/SAI1
        let ping = cortex_m::singleton!(: [SampleType; BUFFER_LEN] = [0; BUFFER_LEN]).unwrap();
        let pong = cortex_m::singleton!(: [SampleType; BUFFER_LEN] = [0; BUFFER_LEN]).unwrap();

        // setup_dma_double_buffer(&mut sai, &mut dma, ping, pong);

        // SAI
        let mclk = gpioe.pe2.into_alternate::<6>(); // MCLK
        let sck = gpioe.pe5.into_alternate::<6>(); // SCK
        let fs = gpioe.pe4.into_alternate::<6>(); // WS
        let sd = gpioe.pe6.into_alternate::<6>(); // SD
        let sd2: Option<gpio::PE3<gpio::Alternate<6>>> = None; // SD2

        let mut sai = sai::Sai::i2s_sai1_ch_a(
            dp.SAI1,
            (mclk, sck, fs, sd, sd2),
            sample_rate,
            sai::I2SDataSize::BITS_16,
            ccdr.peripheral.SAI1,
            &ccdr.clocks,
            sai::I2sUsers::new(sai::I2SChanConfig::new(sai::I2SDir::Tx)),
        );

        // DMA
        let dma = dma::dma::StreamsTuple::new(dp.DMA2, ccdr.peripheral.DMA2);
        let stream = dma.0;

        let config = dma::dma::DmaConfig::default()
            .memory_increment(true)
            .peripheral_increment(false)
            .transfer_complete_interrupt(true)
            .double_buffer(true)
            .priority(dma::config::Priority::High)
            .memory_burst(dma::config::BurstMode::NoBurst)
            .peripheral_burst(dma::config::BurstMode::NoBurst);

        let sai_addr = dsp::audio::SaiTxDma::<SampleType>::new();
        let transfer = dma::Transfer::init(stream, sai_addr, ping, Some(pong), config);

        sai.enable();

        // Spawn tasks
        process_midi_bytes::spawn(midi_rx_recv).unwrap();
        // lcd_task::spawn().unwrap();

        (
            Shared {
                state: State::new(),
                audio_state: AudioState::new(),
            },
            Local {
                // sample_timer,
                midi_rx: rx,
                midi_parser: MidiParser::new(MidiChannel::Ch1),
                midi_rx_send,
                // lcd,
                _sai: sai,
                transfer,
                tmp_buf: cortex_m::singleton!(: [SampleType; BUFFER_LEN] = [0; BUFFER_LEN])
                    .unwrap(),
            },
            // init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    // #[task(binds = TIM2, priority = 9, local = [sample_timer], shared = [state])]
    // fn generator(mut cx: generator::Context) {
    //     cx.local.sample_timer.sr.modify(|_, w| w.uif().clear_bit());

    //     let _sample = cx.shared.state.lock(|state| state.next_sample());
    //     // audio_buffer.push(sample);
    // }

    #[task(binds = USART2, priority = 8, local = [midi_rx, midi_rx_send])]
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

    #[task(binds = DMA2_STR0, priority = 9, local = [transfer, tmp_buf], shared = [audio_state])]
    fn i2s_stream(mut cx: i2s_stream::Context) {
        let tmp_buf: &'static mut [SampleType; BUFFER_LEN] =
            unsafe { core::ptr::read(cx.local.tmp_buf) };

        match cx.local.transfer.next_transfer(tmp_buf) {
            Ok((buf_old, _current_buffer, _transfer_size)) => {
                cx.shared.audio_state.lock(|state| {
                    for i in 0..(buf_old.len() / 2) {
                        let (l, r) = state.next_sample_stereo();
                        buf_old[i * 2] = l;
                        buf_old[i * 2 + 1] = r;
                    }
                });

                *cx.local.tmp_buf = buf_old;
            }
            Err(err) => match err {
                dma::DMAError::NotReady => warn!("DMA error: NotReady"),
                dma::DMAError::SmallBuffer => warn!("DMA error: SmallBuffer"),
                dma::DMAError::Overflow => warn!("DMA error: Overflow"),
            },
        }
    }

    // #[task(priority = 7, local = [lcd])]
    // async fn lcd_task(ctx: lcd_task::Context) {
    //     let lcd = ctx.local.lcd;
    //     lcd.init();

    //     lcd.set_row(0);
    //     lcd.write_str("Param:");
    //     lcd.set_row(1);
    //     lcd.write_str("123 Hz");
    // }
}
