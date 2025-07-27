use embedded_dma::Word;
use stm32h7xx_hal::{dma, pac};

pub const BUFFER_LEN: usize = 256;
pub type SampleType = i16;

pub struct AudioState {
    phase: f32,
    freq: f32,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            freq: 440.0,
        }
    }

    pub fn next_sample_stereo(&mut self) -> (i16, i16) {
        self.phase = (self.phase + self.freq / 44100.0) % 1.0;
        let s = libm::sinf(self.phase * 2.0 * core::f32::consts::PI);
        let val = (s * i16::MAX as f32) as i16;
        (val, val) // Same for L/R
    }

    pub fn advance_time(&mut self) {
        // Update freq, phase, etc
    }
}

pub struct SaiTxDma<T>
where
    T: Word,
{
    address: usize,
    _marker: core::marker::PhantomData<T>,
}

impl<T> SaiTxDma<T>
where
    T: Word,
{
    pub fn new() -> Self {
        let dr_ptr = unsafe { &(*pac::SAI1::ptr()).ch[0].dr as *const _ as usize };
        Self {
            address: dr_ptr,
            _marker: core::marker::PhantomData,
        }
    }
}

unsafe impl<T> dma::traits::TargetAddress<dma::MemoryToPeripheral> for SaiTxDma<T>
where
    T: Word,
{
    fn address(&self) -> usize {
        self.address
    }

    type MemSize = T;
}
