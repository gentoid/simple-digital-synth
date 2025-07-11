use core::{
    cell::UnsafeCell,
    sync::atomic::{compiler_fence, AtomicBool, Ordering},
};

use critical_section::RawRestoreState;
use embassy_stm32::{hsem::HardwareSemaphore, peripherals::HSEM};

const SEMAPHORE_ID: u8 = 0;

const PROCESS_ID: u8 = 1;

struct HsemCell {
    inner: UnsafeCell<Option<HardwareSemaphore<'static, HSEM>>>,
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

unsafe impl Sync for HsemCell {}

static HSEM_CELL: HsemCell = HsemCell {
    inner: UnsafeCell::new(None),
};

pub struct HsemCriticalSection;

pub fn init_hsem_driver(hsem: HardwareSemaphore<'static, HSEM>) {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        // already initialized
        return;
    }
    
    unsafe {
        *HSEM_CELL.inner.get() = Some(hsem);
    }
}

fn hsem() -> &'static mut HardwareSemaphore<'static, HSEM> {
    let inner = unsafe { &mut *HSEM_CELL.inner.get() };
    inner.as_mut().expect("HSEM is not initialized")
}

unsafe impl critical_section::Impl for HsemCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let hsem = hsem();

        loop {
            if hsem.two_step_lock(SEMAPHORE_ID, PROCESS_ID).is_ok() {
                compiler_fence(Ordering::SeqCst);
                return;
            }
        }
    }

    unsafe fn release(_restore_state: RawRestoreState) {
        compiler_fence(Ordering::SeqCst);
        hsem().unlock(SEMAPHORE_ID, PROCESS_ID);
    }
}
