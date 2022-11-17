//! RCP - Audio Interface

use core::ops::{Deref, DerefMut};
use proc_bitfield::bitfield;
use crate::{RW, WO};

/// A wrapper around a mutable reference to the Audio Interface's memory mapped registers.
/// 
/// See [`AudioInterface::new()`] for usage details.
pub struct AudioInterface {
    r: &'static mut RegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    pub dram_addr: WO<u32>,
    pub length: RW<u32>,
    pub control: WO<u32>,
    pub status: RW<StatusReg>,
    pub dac_rate: WO<u32>,
    pub bit_rate: WO<u32>,
}
impl AudioInterface {
    /// Creates a new wrapped mutable reference to the Audio Interface's memory mapped registers, starting at `0xA4500000`.
    /// 
    /// Developers are recommended to use [`Hardware::take()`][crate::Hardware::take()] instead.
    /// But for unrestricted, unsafe, access, this struct provides a method-based version to the
    /// static functions available at the [module][crate::ai] level.
    /// 
    /// # Safety
    /// This provides unrestricted access to memory mapped registers. Data races _could_ occur if writing
    /// to a register in both regular code and inside interrupt handlers.
    /// 
    /// This is especially problematic if performing a read-modify-write operation; an interrupt
    /// could trigger between reading a register, and writing a modified value back to the same
    /// register. Thus anything written to that register inside the interrupt, would only apply for
    /// a short moment before being overwritten.
    #[inline(always)]
    pub unsafe fn new() -> Self { Self {
        r: &mut *(0xA4500000 as *mut RegisterBlock)
    }}
}
impl Deref for AudioInterface {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        self.r
    }
}
impl DerefMut for AudioInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.r
    }
}

regfn_wo!(AudioInterface, dram_addr, DRAM_ADDR, u32);
regfn_rw!(AudioInterface, length, LENGTH, u32);
regfn_wo!(AudioInterface, control, CONTROL, u32);
regfn_rw!(AudioInterface, status, STATUS, StatusReg);
regfn_wo!(AudioInterface, dac_rate, DAC_RATE, u32);
regfn_wo!(AudioInterface, bit_rate, BIT_RATE, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StatusReg(pub u32): Debug {
        pub clear_interrupt: u32 [wo] @ ..,
        
        pub full: bool [ro] @ 0, // bit 31 returns the same value; but bit 0 is used because it's more efficient
        pub dac_cntr: u16 [ro] @ 1..=14,
        pub bitclock_state: bool [ro] @ 16,
        pub abus_word_2: bool [ro] @ 19,
        pub word_select: bool [ro] @ 21,
        pub data_available: bool [ro] @ 22,
        pub dfifo2_loaded: bool [ro] @ 23,
        pub dma_enable: bool [ro] @ 25,
        pub dma_request: bool [ro] @ 26,
        pub dma_busy: bool [ro] @ 27,
        pub busy: bool [ro] @ 30,
    }
}