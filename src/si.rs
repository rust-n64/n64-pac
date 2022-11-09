//! RCP - Serial Interface

use core::ops::Deref;
use proc_bitfield::bitfield;
use crate::RW;

/// A wrapper around a mutable reference to the Serial Interface's memory mapped registers.
/// 
/// See [`SerialInterface::new()`] for usage details.
pub struct SerialInterface {
    r: &'static mut RegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    pub dram_addr: RW<u32>,
    pub pif_ad_rd64b: RW<u32>,
    pub pif_ad_wr4b: RW<u32>,
    pub pif_ad_wr64b: RW<u32>,
    pub pif_ad_rd4b: RW<u32>,
    pub status: RW<StatusReg>,
}
impl SerialInterface {
    /// Creates a new wrapped mutable reference to the Serial Interface's memory mapped registers, starting at `0xA4800000`.
    /// 
    /// Developers are recommended to use [`Hardware::take()`][crate::Hardware::take()] instead.
    /// But for unrestricted, unsafe, access, this struct provides a method-based version to the
    /// static functions available at the [module][crate::si] level.
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
        r: &mut *(0xA4800000 as *mut RegisterBlock)
    }}
}
impl Deref for SerialInterface {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        self.r
    }
}

regfn_rw!(SerialInterface, dram_addr, DRAM_ADDR, u32);
regfn_rw!(SerialInterface, pif_ad_rd64b, PIF_AD_RD64B, u32);
regfn_rw!(SerialInterface, pif_ad_wr4b, PIF_AD_WR4B, u32);
regfn_rw!(SerialInterface, pif_ad_wr64b, PIF_AD_WR64B, u32);
regfn_rw!(SerialInterface, pif_ad_rd4b, PIF_AD_RD4B, u32);
regfn_rw!(SerialInterface, status, STATUS, StatusReg);


bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StatusReg(pub u32): Debug {
        pub whole_register: u32 [wo] @ ..,
        
        pub dma_busy: bool [ro] @ 0,
        pub io_busy: bool [ro] @ 1,
        pub read_pending: bool [ro] @ 2,
        pub dma_error: bool [ro] @ 3,
        pub pch_state: u8 [ro] @ 4..=7,
        pub dma_state: u8 [ro] @ 8..=11,
        
        /// Mirror of the SI interrupt flag from the `MI_INTERRUPT` register.
        /// 
        /// Writing any value to the `SI_STATUS` register clears the flag across all three locations
        /// (this bit, `MI_INTERRUPT`, and the RCP Interrupt Cause register).
        /// 
        /// SI Interrupts occur when a DMA write finishes.
        pub interrupt: bool @ 12,
    }
}