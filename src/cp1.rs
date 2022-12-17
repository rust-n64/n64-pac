//! FPU - Coprocessor 1

use core::arch::asm;
use core::marker::PhantomData;
use num_enum::{FromPrimitive, IntoPrimitive};
use proc_bitfield::bitfield;

//TODO: Complete rustdocs for all bitfields

macro_rules! cp1fn_ro {
    ($reg:ident, $width:ident, $index:literal, $datatype:ident) => {
        paste::paste! {
            #[doc = concat!("Reads from CP1 register ", stringify!($index), ".")]
            #[inline(always)]
            pub fn $reg() -> $datatype {
                [<read_ $width>]::<$index>().into()
            }
        }
    };
}
macro_rules! cp1fn_wo {
    ($reg:ident, $width:ident, $index:literal, $datatype:ident) => {
        paste::paste! {
            #[doc = concat!("Writes to CP1 register ", stringify!($index), ".")]
            #[inline(always)]
            pub unsafe fn [<set_ $reg>](data: $datatype) {
                [<write_ $width>]::<$index>(data.into());
            }
        }
    };
}
macro_rules! cp1fn_rw {
    ($reg:ident, $width:ident, $index:literal, $datatype:ident) => {
        cp1fn_ro!($reg, $width, $index, $datatype);
        cp1fn_wo!($reg, $width, $index, $datatype);
        
        paste::paste! {
            #[doc = concat!("Reads from CP1 register ", stringify!($index), ", modifies the data, then writes it back into the register.")]
            #[inline(always)]
            pub unsafe fn [<modify_ $reg>]<F: FnOnce($datatype) -> $datatype>(func: F) {
                [<set_ $reg>](func($reg()));
            }
        }
    }
}

/// A zero-sized struct for accessing CP1 registers via methods.
/// 
/// See [`Cp1::new()`] for usage details.
pub struct Cp1 {
    _marker: PhantomData<*const ()>
}
impl Cp1 {
    /// Creates a new zero-sized struct providing access to CP1 registers.
    /// 
    /// Developers are recommended to use [`Hardware::take()`][crate::Hardware::take()] instead.
    /// But for unrestricted, unsafe, access, this struct provides a method-based version to the
    /// static functions available at the [module][crate::cp1] level.
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
    
    cpxmethod_ro!(revision_implementation, ImplementationRevisionReg);
    cpxmethod_rw!(control_status, ControlStatusReg);
}

cp1fn_ro!(revision_implementation, u32, 0, ImplementationRevisionReg);
cp1fn_rw!(control_status, u32, 31, ControlStatusReg);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ImplementationRevisionReg(pub u32): Debug {
        /// Processor revision number
        pub revision: u8 [ro] @ 0..=7,
        /// Likely is 0x0B for all VR4300 series CPUs
        pub implementation: u8 [ro] @ 8..=15,
    }
}
derive_tofrom_primitive!(ImplementationRevisionReg, u32);

#[derive(IntoPrimitive, FromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RoundingMode {
    /// Nearest 
    /// 
    /// Rounds to nearest whole number. If value is midway between upper and lower numbers, it rounds
    /// to nearest even number.
    #[default]
    Rn = 0b00,
    
    /// Zero
    /// 
    /// Truncates fractional part.
    Rz = 0b01,
    
    /// Ceil
    /// 
    /// Rounds toward +infinity.
    Rp = 0b10,
    
    /// Floor
    /// 
    /// Rounds toward -infinity.
    Rm = 0b11,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ControlStatusReg(pub u32): Debug {
        /// Rounding mode used for all float operations
        pub rm: u8 [RoundingMode] @ 0..=1,
        
        pub flags: u8 @ 2..=6,
        pub flag_inexact: bool @ 2,
        pub flag_underflow: bool @ 3,
        pub flag_overflow: bool @ 4,
        pub flag_divzero: bool @ 5,
        pub flag_invalid: bool @ 6,
        
        pub enables: u8 @ 7..=11,
        pub enable_inexact: bool @ 7,
        pub enable_underflow: bool @ 8,
        pub enable_overflow: bool @ 9,
        pub enable_divzero: bool @ 10,
        pub enable_invalid: bool @ 11,
        
        pub causes: u8 @ 12..=17,
        pub cause_inexact: bool @ 12,
        pub cause_underflow: bool @ 13,
        pub cause_overflow: bool @ 14,
        pub cause_divzero: bool @ 15,
        pub cause_invalid: bool @ 16,
        pub cause_unimplemented: bool @ 17,
        
        /// Condition is set based on the result of a floating-point Compare operation
        pub c: bool @ 23,
        
        /// Determines result when flushing denormalized number to zero (more details: <https://n64brew.dev/wiki/COP1>)
        pub fs: bool @ 24,
    }
}
derive_tofrom_primitive!(ControlStatusReg, u32);



#[inline(always)]
pub fn read_u32<const INDEX: u32>() -> u32 {
    let value: u32;
    unsafe {
        asm!("
            .set noat
            mfc1 {gpr}, ${cp_reg}
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
            dmfc1 {tmp}, ${cp_reg}
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

/// Read CP1 control register
/// 
/// Only registers 0 (Implementation/Revision) and 31 (Control/Status) are known to exist.
#[inline(always)]
pub fn read_fcr<const INDEX: u32>() -> u32 {
    let value: u32;
    unsafe {
        asm!("
            .set noat
            cfc1 {gpr}, ${cp_reg}
        ",
        gpr = out(reg) value,
        cp_reg = const INDEX
        );
    }
    
    value
}

#[inline(always)]
pub unsafe fn write_u32<const INDEX: u32>(value: u32) {
    asm!("
        .set noat
        mtc1 {gpr}, ${cp_reg}
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
        dmtc1 {tmp}, ${cp_reg}
        nop
    ",
    tmp = out(reg) _,
    tmp2 = out(reg) _,
    lo = in(reg) (value as u32),
    hi = in(reg) ((value >> 32) as u32),
    cp_reg = const INDEX
    );
}

/// Write CP1 control register
/// 
/// Only registers 0 (Implementation/Revision) and 31 (Control/Status) are known to exist.
#[inline(always)]
pub unsafe fn write_fcr<const INDEX: u32>(value: u32) {
    asm!("
        .set noat
        mtc1 {gpr}, ${cp_reg}
        nop
    ",
    gpr = in(reg) value,
    cp_reg = const INDEX
    );
}
