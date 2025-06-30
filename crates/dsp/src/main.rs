#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::{Mutex, free};
use cortex_m_rt::entry;
use defmt::*;
use stm32h7xx_hal::{
    dma::{dma::{Stream0}, MemoryToPeripheral, Transfer},
    gpio::{Alternate, PA4, PA5, PA7},
    interrupt, pac,
    prelude::*,
    rcc::{self, ResetEnable},
    sai,
};

pub mod adsr;
pub mod filter;
pub mod oscillator;
pub mod state;
pub mod voice;

pub mod consts;

use crate::{
    consts::{MAX_DAC_VALUE, SAMPLE_RATE},
    state::State,
};

type I2sPins = (
    PA5<Alternate<5>>, //SCK
    PA4<Alternate<5>>, // WS (LRCK)
    PA7<Alternate<5>>, // MOSI (SD)
);

const SINE_SAMPLES: usize = 96;
static STATE: Mutex<RefCell<State>> = Mutex::new(RefCell::new(State::new()));
static DMA_TRANSFER: Mutex<
    RefCell<
        Option<
            Transfer<
                Stream0<pac::DMA1>,
                pac::SPI1,
                MemoryToPeripheral,
                &'static mut [u16; SINE_SAMPLES],
                u32,
            >,
        >,
    >,
> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    info!("Starting the DSP core");

    let dp = pac::Peripherals::take().unwrap();
    // let cp = cortex_m::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(200.MHz()).freeze(vos, &dp.SYSCFG);

    // I2S

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let pins: I2sPins = (
        gpioa.pa5.into_alternate::<5>(),
        gpioa.pa4.into_alternate::<5>(),
        gpioa.pa7.into_alternate::<5>(),
    );

    let rec_spi1 = ccdr.peripheral.SPI1;
    let i2s = sai::I2S::new(sai::I2SChanConfig::new(sai::I2SDir::Tx));

    // Tim2

    let rec_tim2 = ccdr.peripheral.TIM2;
    rec_tim2.enable();

    setup_tim2(&dp.TIM2, &ccdr.clocks);

    loop {
        cortex_m::asm::wfi();
    }
}

fn setup_tim2(tim: &pac::TIM2, clocks: &rcc::CoreClocks) {
    let target_freq = SAMPLE_RATE as u64;

    let pclk = clocks.pclk1().to_Hz() as u64;

    let mut best_psc = 0;
    let mut best_arr = 0;
    let mut min_error = u32::MAX;

    for psc in 0..=u16::MAX as u64 {
        let arr_candidate = pclk / ((psc as u64 + 1) * target_freq);
        if arr_candidate == 0 || arr_candidate > u32::MAX as u64 {
            continue;
        }

        let actual = pclk / ((psc + 1) * arr_candidate);
        let error = (actual as i32 - target_freq as i32).unsigned_abs();

        if error < min_error {
            min_error = error;
            best_psc = psc;
            best_arr = arr_candidate;

            if error == 0 {
                break;
            }
        }
    }

    // Turn it off before setting up
    tim.cr1.modify(|_, w| w.cen().clear_bit());

    tim.psc.write(|w| w.psc().bits(best_psc as u16));
    tim.arr.write(|w| w.arr().bits((best_arr - 1) as u32));

    tim.egr.write(|w| w.ug().set_bit());

    tim.dier.write(|w| w.uie().set_bit());
    tim.cr1.write(|w| w.cen().set_bit());

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIM2);
    }
}

#[interrupt]
fn TIM2() {
    let dp = unsafe { pac::Peripherals::steal() };
    let tim = &dp.TIM2;
    tim.sr.modify(|_, w| w.uif().clear_bit());

    free(|cs| {
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
