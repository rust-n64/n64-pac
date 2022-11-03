
//! Low-level hardware abstraction crate (aka a Peripheral Access Crate) for the Nintendo 64 console.
//! 
//! Unlike typical PACs, this API is not generated via svd2rust, as it doesn't support the architecture
//! and the N64 contains features not found on microcontrollers.
//! 
//! Additionally, all parts of the API are freely accessible without any concept of ownership. It is
//! very likely that software running on the N64 may require shared access to various parts of the
//! hardware. Forcing developers to pass around a reference to the entire PAC decreases usability,
//! and does not necessarily ensure safety.
//! 
//! # Safety
//! Functions in this PAC are marked unsafe if the operation itself could actively cause unexpected
//! or undefined behavior, such as an exception or other form of failure.
//! 
//! It is possible that a write or read-modify-write operation could be unsafe, but only if used in
//! an async environment or when the same register is accessed from both regular code and from
//! within an interrupt handler. The PAC is not responsible for either situation.
//! 
//! # Memory Mapped Registers
//! There are two distinct methods for accessing and modifying these registers.
//! 
//! ##### Static functions
//! Each module containing memory mapped registers, will provide static functions for each, where
//! applicable (e.g. read-only registers won't have a `set_` function).
//! 
//! This is the recommended method for low-level interaction with registers.
//! ```
//! use n64_pac::vi;
//! use n64_pac::vi::ColorDepth;
//!
//! let mut value = vi::ctrl();
//! value.set_depth(ColorDepth::BPP32);
//! vi::set_ctrl(value);
//! ```
//! For read-modify-write operations, a `modify()` function is also available:
//! ```
//! use n64_pac::vi;
//! use n64_pac::vi::ColorDepth;
//!
//! vi::modify_ctrl(|reg| reg.with_depth(ColorDepth::BPP32));
//! ```
//! 
//! ##### Local RegisterBlock wrapper object
//! Alternatively, the wrappers implicitly used by the above static functions, can be created locally. 
//! 
//! This is not any more or less efficient than the static functions, but it provides a tangible
//! representation of the registers in memory. This could be useful for HALs that want to hide
//! PAC-level access inside its own types.
//! ```
//! use n64_pac::vi::{ColorDepth, VideoInterface};
//!
//! let vi = VideoInterface::new();
//! let mut value = vi.ctrl.read();
//! value.set_depth(ColorDepth::BPP32);
//! vi.ctrl.write(value);
//! ```
//! For read-modify-write operations, a `modify()` function is also available:
//! ```
//! use n64_pac::vi::{ColorDepth, VideoInterface};
//!
//! let vi = VideoInterface::new();
//! vi.ctrl.modify(|reg| reg.with_depth(ColorDepth::BPP32));
//! ```
//! When the wrapper goes out of scope, the _reference_ will be dropped. The static lifetime attached
//! to the reference only indicates that the "data" (memory mapped registers) it points to will live forever.
//! 
//! # CPU Configuration Registers
//! These registers are not mapped to memory, and instead require special assembly instructions to access.
//! 
//! To access these registers, there are static functions available in modules [`cp0`] and [`cp1`].
//! ```
//! use n64_pac::cp0;
//! 
//! let mut value = cp0::status();
//! value.set_ie(true);
//! cp0::set_status(value);
//! ```
//! For read-modify-write operations, a `modify()` function is also available:
//! ```
//! use n64_pac::cp0;
//! 
//! cp0::modify_status(|reg| reg.with_ie(true));
//! ```
//! 

#![no_std]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

macro_rules! regfn_ro {
    ($block:ident, $reg:ident, $reg_name:expr, $datatype:ident) => {
        #[doc = concat!("Creates a temporary pointer to the [`", stringify!($block), "`], and reads data from its ", stringify!($reg_name), " register.")]
        #[inline(always)]
        pub fn $reg() -> $datatype {
            $block::new().$reg.read()
        }
    };
}
macro_rules! regfn_wo {
    ($block:ident, $reg:ident, $reg_name:expr, $datatype:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a temporary pointer to the [`", stringify!($block), "`], and writes data to its ", stringify!($reg_name), " register.")]
            #[inline(always)]
            pub fn [<set_ $reg>](data: $datatype) {
                $block::new().$reg.write(data);
            }
        }
    }
}
macro_rules! regfn_rw {
    ($block:ident, $reg:ident, $reg_name:expr, $datatype:ident) => {
        regfn_ro!($block, $reg, $reg_name, $datatype);
        regfn_wo!($block, $reg, $reg_name, $datatype);
        
        paste::paste! {
            #[doc = concat!("Creates a temporary pointer to the [`", stringify!($block), "`], reads data from its ", stringify!($reg_name), " register, modifies the data, then finally writes back into the register.")]
            #[inline(always)]
            pub fn [<modify_ $reg>]<F: FnOnce($datatype) -> $datatype>(func: F) {
                $block::new().$reg.modify(func);
            }
        }
    }
}

macro_rules! regfn_ro_union {
    ($block:ident, $reg:ident, $reg_name:expr, $uniontype:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a temporary pointer to the [`", stringify!($block), "`], and reads data from its ", stringify!($reg_name), " register.")]
            #[inline(always)]
            pub fn $reg() -> [<$uniontype Read>] {
                unsafe { $block::new().$reg.read().read }
            }
        }
    };
}
macro_rules! regfn_wo_union {
    ($block:ident, $reg:ident, $reg_name:expr, $uniontype:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a temporary pointer to the [`", stringify!($block), "`], and writes data to its ", stringify!($reg_name), " register.")]
            #[inline(always)]
            pub fn [<set_ $reg>](data: [<$uniontype Write>]) {
                $block::new().$reg.write($uniontype { write: data });
            }
        }
    }
}
macro_rules! regfn_rw_union {
    ($block:ident, $reg:ident, $reg_name:expr, $uniontype:ident) => {
        regfn_ro_union!($block, $reg, $reg_name, $uniontype);
        regfn_wo_union!($block, $reg, $reg_name, $uniontype);
    }
}

pub mod cp0;
pub mod mi;
pub mod si;
pub mod vi;

pub struct RW<T: Copy>(T);
impl<T: Copy> RW<T> {
    /// Reads the value this struct represents from memory.
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { (&self.0 as *const T).read_volatile() }
    }
    
    /// Writes the provided value to the memory represented by this struct.
    /// 
    /// # Safety
    /// While the function itself is safe, using it to modify a previously read value from the same
    /// struct, could be unsafe if interrupts are enabled.
    #[inline(always)] 
    pub fn write(&self, data: T) {
        unsafe { (&self.0 as *const T as *mut T).write_volatile(data); }
    }
    
    /// Reads the value this struct represents from memory, executes the provided function, and
    /// writes the resulting value back to memory.
    /// 
    /// # Safety
    /// Unsafe when interrupts are enabled, as they could interrupt between this function reading
    /// the data, and writing the modified data back.
    #[inline(always)]
    pub fn modify<F: FnOnce(T) -> T>(&self, func: F) {
        unsafe {
            let ptr = &self.0 as *const T as *mut T;
            ptr.write_volatile(func(ptr.read_volatile()));
        }
    }
}

pub struct RO<T: Copy>(T);
impl<T: Copy> RO<T> {
    /// Reads the value this struct represents from memory.
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { (&self.0 as *const T).read_volatile() }
    }
}

pub struct WO<T: Copy>(T);
impl<T: Copy> WO<T> {
    /// Writes the provided value to the memory represented by this struct.
    #[inline(always)]
    pub fn write(&mut self, data: T) {
        unsafe { (&mut self.0 as *mut T).write_volatile(data); }
    }
}