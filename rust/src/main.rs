#![no_main]
#![no_std]

use core::{
    cell::RefCell,
    sync::atomic::{AtomicU16, Ordering},
};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{
    gpio::{Edge, Input, PB0, PB1},
    interrupt, pac,
    prelude::*,
};

pub mod consts;
pub mod midi;
pub mod tables;

use crate::{consts::MIDI_NOTES_AMOUNT, tables::sine::SINE_WAVE};

// for Encoder

static CLK_PIN: Mutex<RefCell<Option<PB0<Input>>>> = Mutex::new(RefCell::new(None));
static DT_PIN: Mutex<RefCell<Option<PB1<Input>>>> = Mutex::new(RefCell::new(None));

static MIDI_NOTE: AtomicU16 = AtomicU16::new(69); // A1, 440Hz

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();

    let rcc_regs = dp.RCC;
    rcc_regs.ahbenr.modify(|_, w| w.dma2en().enabled());
    rcc_regs.apb1enr.modify(|_, w| {
        w.dac1en().enabled();
        w.tim7en().set_bit()
    });

    let mut rcc = rcc_regs.constrain();

    let mut flash = dp.FLASH.constrain();
    let _clocks = rcc.cfgr.sysclk(40.MHz()).freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let _pa4 = gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
    let dma = dp.DMA2;

    setup_dac_dma(&dp.TIM7, &dp.DAC1, &dma.ch3);

    // // Encoder

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let mut clk = gpiob
        .pb0
        .into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let dt = gpiob
        .pb1
        .into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);

    let mut syscfg = dp.SYSCFG.constrain(&mut rcc.apb2);
    syscfg.select_exti_interrupt_source(&clk);

    clk.enable_interrupt(&mut dp.EXTI);
    clk.trigger_on_edge(&mut dp.EXTI, Edge::Falling);

    cortex_m::interrupt::free(|cs| {
        CLK_PIN.borrow(cs).replace(Some(clk));
        DT_PIN.borrow(cs).replace(Some(dt));
    });

    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI0);
    }

    loop {
        cortex_m::asm::wfi();
    }
}

#[derive(PartialEq)]
enum Rotation {
    Left,
    Right,
}

#[interrupt]
fn EXTI0() {
    cortex_m::interrupt::free(|cs| {
        let mut clk_pin = CLK_PIN.borrow(cs).borrow_mut();
        let dt_pin = DT_PIN.borrow(cs).borrow();

        if let (Some(clk), Some(dt)) = (clk_pin.as_mut(), dt_pin.as_ref()) {
            clk.clear_interrupt();

            let dir = dt.is_high().unwrap_or(false);
            let diff = if dir { Rotation::Left } else { Rotation::Right };
            update_count(diff);
        }
    });
}

fn update_count(direction: Rotation) {
    MIDI_NOTE
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            let new = if direction == Rotation::Right {
                current.saturating_add(1)
            } else {
                current.saturating_sub(1)
            };
            Some(new.clamp(0, MIDI_NOTES_AMOUNT as u16 - 1))
        })
        .ok();
}

fn setup_dac_dma(tim: &pac::TIM7, dac: &pac::DAC1, dma: &pac::dma2::CH) {
    dma.mar
        .write(|w| unsafe { w.ma().bits(SINE_WAVE.as_ptr() as u32) });
    dma.par.write(|w| unsafe { w.pa().bits(0x40007408) });
    dma.ndtr.write(|w| w.ndt().bits(SINE_WAVE.len() as u16));
    dma.cr.write(|w| {
        w.mem2mem().disabled();
        w.pl().high();
        w.msize().bits16();
        w.psize().bits16();
        w.minc().enabled();
        w.pinc().disabled();
        w.dir().from_memory();
        w.circ().enabled();
        w.en().enabled()
    });

    tim.psc.write(|w| w.psc().bits(39));
    tim.arr.write(|w| w.arr().bits(255));
    tim.cr2.write(|w| w.mms().update());
    tim.cr1.modify(|_, w| w.cen().set_bit());

    dac.cr.modify(|_, w| {
        w.ten1().enabled();
        w.tsel1().tim7_trgo();
        w.dmaen1().enabled();
        w.en1().enabled()
    });
}
