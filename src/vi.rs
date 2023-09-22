//! RCP - Video Interface

use core::ops::Deref;
use num_enum::{FromPrimitive, IntoPrimitive};
use proc_bitfield::bitfield;
use crate::RW;

/// A wrapper around a mutable reference to the Video Interface's memory mapped registers.
/// 
/// See [`VideoInterface::new()`] for usage details.
pub struct VideoInterface {
    r: &'static mut RegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    pub ctrl: RW<CtrlReg>,
    pub origin: RW<u32>,
    pub width: RW<u32>,
    pub v_intr: RW<u32>,
    pub v_current: RW<u32>,
    pub burst: RW<BurstReg>,
    pub v_sync: RW<u32>,
    pub h_sync: RW<HSyncReg>,
    pub h_sync_leap: RW<HSyncLeapReg>,
    pub h_video: RW<HVideoReg>,
    pub v_video: RW<VVideoReg>,
    pub v_burst: RW<VBurstReg>,
    pub x_scale: RW<XScaleReg>,
    pub y_scale: RW<YScaleReg>,
    pub test_addr: RW<u32>,
    pub staged_data: RW<u32>,
}
impl VideoInterface {
    /// Creates a new wrapped mutable reference to the Video Interface's memory mapped registers, starting at `0xA4400000`.
    /// 
    /// Developers are recommended to use [`Hardware::take()`][crate::Hardware::take()] instead.
    /// But for unrestricted, unsafe, access, this struct provides a method-based version to the
    /// static functions available at the [module][crate::vi] level.
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
        r: &mut *(0xA4400000 as *mut RegisterBlock)
    }}
}
impl Deref for VideoInterface {
    type Target = RegisterBlock;
    
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.r
    }
}

regfn_rw!(VideoInterface, ctrl, CTRL, CtrlReg);
regfn_rw!(VideoInterface, origin, ORIGIN, u32);
regfn_rw!(VideoInterface, width, WIDTH, u32);
regfn_rw!(VideoInterface, v_intr, V_INTR, u32);
regfn_rw!(VideoInterface, v_current, V_CURRENT, u32);
regfn_rw!(VideoInterface, burst, BURST, BurstReg);
regfn_rw!(VideoInterface, v_sync, V_SYNC, u32);
regfn_rw!(VideoInterface, h_sync, H_SYNC, HSyncReg);
regfn_rw!(VideoInterface, h_sync_leap, H_SYNC_LEAP, HSyncLeapReg);
regfn_rw!(VideoInterface, h_video, H_VIDEO, HVideoReg);
regfn_rw!(VideoInterface, v_video, V_VIDEO, VVideoReg);
regfn_rw!(VideoInterface, v_burst, V_BURST, VBurstReg);
regfn_rw!(VideoInterface, x_scale, X_SCALE, XScaleReg);
regfn_rw!(VideoInterface, y_scale, Y_SCALE, YScaleReg);
regfn_rw!(VideoInterface, test_addr, TEST_ADDR, u32);
regfn_rw!(VideoInterface, staged_data, STAGED_DATA, u32);


#[derive(IntoPrimitive, FromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AntiAliasMode {
    Disabled = 3,
    ResamplingOnly = 2,
    EnabledAsNeeded = 1,
    #[default]
    Enabled = 0,
}

#[derive(IntoPrimitive, FromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ColorDepth {
    BPP32 = 3,
    BPP16 = 2,
    Reserved = 1,
    #[default]
    Blank = 0,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct CtrlReg(pub u32): Debug {
        pub depth: u8 [ColorDepth] @ 0..=1,
        pub gamma_dither_enable: bool @ 2,
        pub gamma_enable: bool @ 3,
        pub divot_enable: bool @ 4,
        /// # Safety:
        /// **Never** enable this bit! Early research indicates this could potentially damage the console if set to `true`.
        pub vbus_clock_enable: bool @ 5,
        pub serrate: bool @ 6,
        pub test_mode: bool @ 7,
        pub aa_mode: u8 [AntiAliasMode] @ 8..=9,
        pub kill_we: bool @ 11,
        pub pixel_advance: u8 @ 12..=15,
        pub dither_filter_enable: bool @ 16,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct BurstReg(pub u32): Debug {
        pub hsync_width: u8 @ 0..=7,
        pub burst_width: u8 @ 8..=15,
        pub vsync_width: u8 @ 16..=19,
        pub burst_start: u16 @ 20..=29,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct HSyncReg(pub u32): Debug {
        pub h_sync: u16 @ 0..=11,
        pub leap: u8 @ 16..=20,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct HSyncLeapReg(pub u32): Debug {
        pub leap_b: u16 @ 0..=9,
        pub leap_a: u16 @ 16..=25,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct HVideoReg(pub u32): Debug {
        pub h_end: u16 @ 0..=9,
        pub h_start: u16 @ 16..=25,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct VVideoReg(pub u32): Debug {
        pub v_end: u16 @ 0..=9,
        pub v_start: u16 @ 16..=25,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct VBurstReg(pub u32): Debug {
        pub v_burst_end: u16 @ 0..=9,
        pub v_burst_start: u16 @ 16..=25,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct XScaleReg(pub u32): Debug {
        pub x_scale: u16 @ 0..=11,
        pub x_offset: u16 @ 16..=27,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct YScaleReg(pub u32): Debug {
        pub y_scale: u16 @ 0..=11,
        pub y_offset: u16 @ 16..=27,
    }
}