//! RCP - Peripheral Interface

use core::ops::Deref;
use proc_bitfield::bitfield;
use crate::RW;

/// A wrapper around a mutable reference to the Peripheral Interface's memory mapped registers.
///
/// See [`PeripheralInterface::new()`] for usage details.
pub struct PeripheralInterface {
    r: &'static mut RegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    pub dram_addr: RW<u32>,
    pub cart_addr: RW<u32>,
    pub rd_len: RW<u32>,
    pub wr_len: RW<u32>,
    pub status: RW<StatusReg>,
    pub dom1_lat: RW<u32>,
    pub dom1_pwd: RW<u32>,
    pub dom1_pgs: RW<u32>,
    pub dom1_rls: RW<u32>,
    pub dom2_lat: RW<u32>,
    pub dom2_pwd: RW<u32>,
    pub dom2_pgs: RW<u32>,
    pub dom2_rls: RW<u32>,
}
impl PeripheralInterface {
    /// Creates a new wrapped mutable reference to the Peripheral Interface's memory mapped registers, starting at `0xA4600000`.
    ///
    /// Developers are recommended to use [`Hardware::take()`][crate::Hardware::take()] instead.
    /// But for unrestricted, unsafe, access, this struct provides a method-based version to the
    /// static functions available at the [module][crate::pi] level.
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
        r: &mut *(0xA4600000 as *mut RegisterBlock)
    }}
}
impl Deref for PeripheralInterface {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        self.r
    }
}

regfn_rw_union!(PeripheralInterface, status, STATUS, StatusReg);

#[derive(Copy, Clone)]
#[repr(C)]
pub union StatusReg {
    pub raw: u32,
    pub read: StatusRegRead,
    pub write: StatusRegWrite,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StatusRegRead(pub u32): Debug {
        pub dma_busy: bool [ro] @ 0,
        pub io_busy: bool [ro] @ 1,
        pub dma_error: bool [ro] @ 2,
        pub interrupt: bool [ro] @ 3,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StatusRegWrite(pub u32): Debug {
        clear_interrupt: bool [wo] @ 0,
        reset_dma: bool [wo] @ 1,
    }
}
impl StatusRegWrite {
    #[inline(always)]
    pub fn clear_interrupt(self) -> Self {
        self.with_clear_interrupt(true)
    }

    #[inline(always)]
    pub fn reset_dma(self) -> Self {
        self.with_reset_dma(true)
    }
}