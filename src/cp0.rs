//! CPU - Coprocessor 0

use core::arch::asm;
use core::marker::PhantomData;
use proc_bitfield::bitfield;

//TODO: Add remaining CP0 registers
//TODO: Complete rustdocs for all bitfields

macro_rules! cp0fn_ro {
    ($reg:ident, $width:ident, $index:literal, $datatype:ident) => {
        paste::paste! {
            #[doc = concat!("Reads from CP0 register ", stringify!($index), ".")]
            #[inline(always)]
            pub fn $reg() -> $datatype {
                $datatype([<read_ $width>]::<$index>())
            }
        }
    };
}
macro_rules! cp0fn_wo {
    ($reg:ident, $width:ident, $index:literal, $datatype:ident) => {
        paste::paste! {
            #[doc = concat!("Writes to CP0 register ", stringify!($index), ".")]
            #[inline(always)]
            pub unsafe fn [<set_ $reg>](data: $datatype) {
                [<write_ $width>]::<$index>(data.0);
            }
        }
    };
}
macro_rules! cp0fn_rw {
    ($reg:ident, $width:ident, $index:literal, $datatype:ident) => {
        cp0fn_ro!($reg, $width, $index, $datatype);
        cp0fn_wo!($reg, $width, $index, $datatype);
        
        paste::paste! {
            #[doc = concat!("Reads from CP0 register ", stringify!($index), ", modifies the data, then writes it back into the register.")]
            #[inline(always)]
            pub unsafe fn [<modify_ $reg>]<F: FnOnce($datatype) -> $datatype>(func: F) {
                [<set_ $reg>](func($reg()));
            }
        }
    }
}

macro_rules! cp0method_ro {
    ($reg:ident, $datatype:ident) => {
        pub fn $reg(&self) -> $datatype {
            $reg()
        }
    }
}
macro_rules! cp0method_wo {
    ($reg:ident, $datatype:ident) => {
        paste::paste! {
            pub fn [<set_ $reg>](&self, data: $datatype) {
                unsafe { [<set_ $reg>](data); }
            }
        }
    }
}
macro_rules! cp0method_rw {
    ($reg:ident, $datatype:ident) => {
        cp0method_ro!($reg, $datatype);
        cp0method_wo!($reg, $datatype);
        
        paste::paste! {
            pub fn [<modify_ $reg>]<F: FnOnce($datatype) -> $datatype>(&self, func: F) {
                unsafe { [<set_ $reg>](func($reg())); }
            }
        }
    }
}

/// A zero-sized struct for accessing CP0 register via methods.
/// 
/// See [`Cp0::new()`] for usage details.
pub struct Cp0 {
    _marker: PhantomData<*const ()>
}
impl Cp0 {
    /// Creates a new zero-sized struct providing access to CP0 registers.
    /// 
    /// Developers are recommended to use [`Hardware::take()`][crate::Hardware::take()] instead.
    /// But for unrestricted, unsafe, access, this struct provides a method-based version to the
    /// static functions available at the [module][crate::cp0] level.
    /// 
    /// # Safety
    /// This provides unrestricted access to memory mapped registers. Data races _could_ occur if writing
    /// to a register in both regular code and inside interrupt handlers.
    /// 
    /// This is especially problematic if performing a read-modify-write operation; an interrupt
    /// could trigger between reading a register, and writing a modified value back to the same
    /// register. Thus anything written to that register inside the interrupt, would only apply for
    /// a short moment before being overwritten.
    pub unsafe fn new() -> Self { Self {
        _marker: PhantomData
    }}
    
    cp0method_rw!(status, StatusReg);
}

cp0fn_rw!(status, u32, 12, StatusReg);


bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StatusReg(pub u32): Debug {
        /// Global Interrupt Enable
        /// - 0 = Disabled
        /// - 1 = Enabled
        pub ie: bool @ 0,
        
        /// Exception Level
        /// 
        /// - 0 = Normal
        /// - 1 = Exception
        pub exl: bool @ 1,
        
        /// Error Level
        /// 
        /// - 0 = Normal
        /// - 1 = Error
        pub erl: bool @ 2,
        
        /// Kernel / Supervisor / User mode select
        /// 
        /// - 0 = Kernel
        /// - 1 = Supervisor
        /// - 2 = User
        /// - 3 = Unknown
        pub ksu: u8 @ 3..=4,
        
        /// User addressing/operating mode select.
        /// 
        /// - 0 = 32-bit addressing, 32-bit operations
        /// - 1 = 64-bit addressing, 64-bit operations
        pub ux: bool @ 5,
        
        /// Supervisor addressing/operating mode select.
        /// 
        /// - 0 = 32-bit addressing, 32-bit operations
        /// - 1 = 64-bit addressing, 64-bit operations
        pub sx: bool @ 6,
        
        /// Kernel operating mode select. Note when in Kernel mode, 64-bit operating mode is always active.
        /// 
        /// - 0 = 32-bit addressing, 64-bit operations
        /// - 1 = 64-bit addressing, 64-bit operations
        pub kx: bool @ 7,
        pub im: u8 @ 8..=15,
        pub im_ip0: bool @ 8,
        pub im_ip1: bool @ 9,
        pub im_int0: bool @ 10,
        pub im_int1: bool @ 11,
        pub im_int2: bool @ 12,
        pub im_int3: bool @ 13,
        pub im_int4: bool @ 14,
        pub im_timer: bool @ 15,
        pub ds: u16 @ 16..=24,
        pub ds_de: bool @ 16,
        pub ds_ce: bool @ 17,
        pub ds_ch: bool @ 18,
        // const 0 @ 19
        pub ds_sr: bool @ 20,
        pub ds_ts: bool @ 21,
        pub ds_bev: bool @ 22,
        // const 0 @ 23
        pub ds_its: bool @ 24,
        pub re: bool @ 25,
        pub fr: bool @ 26,
        pub rp: bool @ 27,
        pub cu: u8 @ 28..=31,
    }
}



#[inline(always)]
pub fn read_u32<const INDEX: u32>() -> u32 {
    let value: u32;
    unsafe {
        asm!("
            .set noat
            mfc0 {gpr}, ${cp_reg}
        ",
        gpr = out(reg) value,
        cp_reg = const INDEX
        );
    }
    
    value
}

#[inline(always)]
pub fn read_u64<const INDEX: u32>() -> u64 {
    let value_lo: u32;
    let value_hi: u32;
    unsafe {
        asm!("
            .set noat
            dmfc0 {tmp}, ${cp_reg}
            add {lo}, $0, {tmp}
            dsrl32 {hi}, {tmp}, 0
        ",
        tmp = out(reg) _,
        lo = out(reg) value_lo,
        hi = out(reg) value_hi,
        cp_reg = const INDEX
        );
    }
    
    ((value_hi as u64) << 32) | (value_lo as u64)
}

#[inline(always)]
pub unsafe fn write_u32<const INDEX: u32>(value: u32) {
    asm!("
        .set noat
        mtc0 {gpr}, ${cp_reg}
        nop
    ",
    gpr = in(reg) value,
    cp_reg = const INDEX
    );
}

#[inline(always)]
pub unsafe fn write_u64<const INDEX: u32>(value: u64) {
    asm!("
        .set noat
        dsll32 {tmp}, {hi}, 0
        dsll32 {tmp2}, {lo}, 0
        dsrl32 {tmp2}, {tmp2}, 0
        or {tmp}, {tmp}, {tmp2}
        dmtc0 {tmp}, ${cp_reg}
        nop
    ",
    tmp = out(reg) _,
    tmp2 = out(reg) _,
    lo = in(reg) (value as u32),
    hi = in(reg) ((value >> 32) as u32),
    cp_reg = const INDEX
    );
}