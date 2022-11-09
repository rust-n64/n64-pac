//! RCP - MIPS Interface

use core::ops::Deref;
use proc_bitfield::bitfield;
use crate::{RO, RW};

/// A wrapper around a mutable reference to the MIPS Interface's memory mapped registers.
/// 
/// See [`MipsInterface::new()`] for usage details.
pub struct MipsInterface {
    r: &'static mut RegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    pub mode: RW<ModeReg>,
    pub version: RO<VersionReg>,
    pub interrupt: RO<InterruptReg>,
    pub mask: RW<MaskReg>,
}
impl MipsInterface {
    /// Creates a new wrapped mutable reference to the MIPS Interface's memory mapped registers, starting at `0xA4300000`.
    /// 
    /// Developers are recommended to use [`Hardware::take()`][crate::Hardware::take()] instead.
    /// But for unrestricted, unsafe, access, this struct provides a method-based version to the
    /// static functions available at the [module][crate::mi] level.
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
        r: &mut *(0xA4300000 as *mut RegisterBlock)
    }}
}
impl Deref for MipsInterface {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        self.r
    }
}

regfn_rw_union!(MipsInterface, mode, MODE, ModeReg);
regfn_ro!(MipsInterface, version, VERSION, VersionReg);
regfn_ro!(MipsInterface, interrupt, INTERRUPT, InterruptReg);
regfn_rw_union!(MipsInterface, mask, MASK, MaskReg);


#[derive(Copy, Clone)]
#[repr(C)]
pub union ModeReg {
    pub raw: u32,
    pub read: ModeRegRead,
    pub write: ModeRegWrite,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ModeRegRead(pub u32): Debug {
        pub init_length: u8 [ro] @ 0..=6,
        pub init_mode: bool [ro] @ 7,
        pub ebus_test_mode: bool [ro] @ 8,
        pub rdram_register_mode: bool [ro] @ 9,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ModeRegWrite(pub u32): Debug {
        pub init_length: u8 [wo] @ 0..=6,
        clear_init_mode: bool [wo] @ 7,
        set_init_mode: bool [wo] @ 8,
        clear_ebus_test_mode: bool [wo] @ 9,
        set_ebus_test_mode: bool [wo] @ 10,
        clear_dp_interrupt: bool [wo] @ 11,
        clear_rdram_register_mode: bool [wo] @ 12,
        set_rdram_register_mode: bool [wo] @ 13,
    }
}
impl ModeRegWrite {
    #[inline(always)]
    pub fn clear_init_mode(self) -> Self {
        self.with_clear_init_mode(true)
    }
    
    #[inline(always)]
    pub fn set_init_mode(self) -> Self {
        self.with_set_init_mode(true)
    }
    
    #[inline(always)]
    pub fn clear_ebus_test_mode(self) -> Self {
        self.with_clear_ebus_test_mode(true)
    }
    
    #[inline(always)]
    pub fn set_ebus_test_mode(self) -> Self {
        self.with_set_ebus_test_mode(true)
    }
    
    #[inline(always)]
    pub fn clear_dp_interrupt(self) -> Self {
        self.with_clear_dp_interrupt(true)
    }
    
    #[inline(always)]
    pub fn clear_rdram_register_mode(self) -> Self {
        self.with_clear_rdram_register_mode(true)
    }
    
    #[inline(always)]
    pub fn set_rdram_register_mode(self) -> Self {
        self.with_set_rdram_register_mode(true)
    }
}



bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct VersionReg(pub u32): Debug {
        pub io_version: u8 [ro] @ 0..=7,
        pub rac_version: u8 [ro] @ 8..=15,
        pub rdp_version: u8 [ro] @ 16..=23,
        pub rsp_version: u8 [ro] @ 24..=31,
    }
}



bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct InterruptReg(pub u32): Debug {
        pub sp: bool [ro] @ 0,
        pub si: bool [ro] @ 1,
        pub ai: bool [ro] @ 2,
        pub vi: bool [ro] @ 3,
        pub pi: bool [ro] @ 4,
        pub dp: bool [ro] @ 5,
    }
}



#[derive(Copy, Clone)]
#[repr(C)]
pub union MaskReg {
    pub raw: u32,
    pub read: MaskRegRead,
    pub write: MaskRegWrite,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct MaskRegRead(pub u32): Debug {
        pub sp_interrupt_mask: bool [ro] @ 0,
        pub si_interrupt_mask: bool [ro] @ 1,
        pub ai_interrupt_mask: bool [ro] @ 2,
        pub vi_interrupt_mask: bool [ro] @ 3,
        pub pi_interrupt_mask: bool [ro] @ 4,
        pub dp_interrupt_mask: bool [ro] @ 5,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct MaskRegWrite(pub u32): Debug {
        clear_sp: bool [wo] @ 0,
        set_sp: bool [wo] @ 1,
        clear_si: bool [wo] @ 2,
        set_si: bool [wo] @ 3,
        clear_ai: bool [wo] @ 4,
        set_ai: bool [wo] @ 5,
        clear_vi: bool [wo] @ 6,
        set_vi: bool [wo] @ 7,
        clear_pi: bool [wo] @ 8,
        set_pi: bool [wo] @ 9,
        clear_dp: bool [wo] @ 10,
        set_dp: bool [wo] @ 11,
    }
}
impl MaskRegWrite {
    #[inline(always)]
    pub fn clear_sp_mask(self) -> Self { self.with_clear_sp(true) }
    #[inline(always)]
    pub fn set_sp_mask(self) -> Self { self.with_set_sp(true) }
    
    #[inline(always)]
    pub fn clear_si_mask(self) -> Self { self.with_clear_si(true) }
    #[inline(always)]
    pub fn set_si_mask(self) -> Self { self.with_set_si(true) }
    
    #[inline(always)]
    pub fn clear_ai_mask(self) -> Self { self.with_clear_ai(true) }
    #[inline(always)]
    pub fn set_ai_mask(self) -> Self { self.with_set_ai(true) }
    
    #[inline(always)]
    pub fn clear_vi_mask(self) -> Self { self.with_clear_vi(true) }
    #[inline(always)]
    pub fn set_vi_mask(self) -> Self { self.with_set_vi(true) }
    
    #[inline(always)]
    pub fn clear_pi_mask(self) -> Self { self.with_clear_pi(true) }
    #[inline(always)]
    pub fn set_pi_mask(self) -> Self { self.with_set_pi(true) }
    
    #[inline(always)]
    pub fn clear_dp_mask(self) -> Self { self.with_clear_dp(true) }
    #[inline(always)]
    pub fn set_dp_mask(self) -> Self { self.with_set_dp(true) }
}
