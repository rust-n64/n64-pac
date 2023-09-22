
//! Low-level hardware abstraction crate (aka a Peripheral Access Crate) for the Nintendo 64 console.
//!
//! Unlike typical PACs, this API is not generated via svd2rust, as it doesn't support the architecture
//! and the N64 contains features not found on microcontrollers.
//!
//! Additionally, while a singleton pattern is available to ensure safe access to hardware, helper
//! functions are available that will bypass this pattern (static write/modify functions require unsafe).
//!
//! # Singleton Pattern
//! [`Hardware`] is a top-level type that holds access to all available hardware abstractions. Using
//! [`Hardware::take()`] a single instance of this abstraction can be taken, and if called a second
//! time, `None` will be returned instead. This ensures safe, race-free, access to low level hardware.
//!
//! If the developer wishes to bypass this pattern, such as in cases where interrupts are not used,
//! or the developer has taken precautions against data races, they have several methods available.
//! - [Static functions](#static-functions)
//! - [Wrapper types](#wrapper-types)
//!
//! # Direct hardware access
//! Both CPU registers and memory mapped registers have two methods of access outside the [`Hardware`]
//! singleton type. Each method will usually optimize down to identical instructions.
//!
//! #### Static functions
//! Each module contains various read/modify/write functions for accessing hardware.
//!
//! ##### Examples
//! Reads the VI_CTRL register, sets the pixel color depth to 32-bits, and writes it back to memory:
//! ```
//! use n64_pac::vi;
//! use n64_pac::vi::ColorDepth;
//!
//! let mut value = vi::ctrl();
//! value.set_depth(ColorDepth::BPP32);
//! unsafe {
//!     vi::set_ctrl(value);
//! }
//! ```
//! Just like the above example, but using the modify function:
//! ```
//! use n64_pac::vi;
//! use n64_pac::vi::ColorDepth;
//!
//! unsafe {
//!     vi::modify_ctrl(|value| value.with_depth(ColorDepth::BPP32));
//! }
//! ```
//!
//! #### Wrapper types
//! Memory mapped registers are accessed using mutable references that point to their location in memory.
//! These references are wrapped into a struct for ease of use and so that blocks of registers can be
//! automatically mapped using only a single pointer address.
//!
//! CPU registers don't use memory locations, but zero-sized structs exist anyways so that they can
//! be accessed via the top-level [`Hardware`] abstraction.
//!
//! It's recommended to use the [Static functions](#static-functions) instead, as they implicitly
//! use these wrappers, and will be optimized into the same instructions.
//!
//! ##### Examples
//! Creates a wrapped pointer to the Video Interface's block of registers, reads the VI_CTRL register,
//! sets the pixel color depth to 32-bits, and writes it back to memory:
//! ```
//! use n64_pac::vi::{ColorDepth, VideoInterface};
//!
//! let vi = unsafe { VideoInterface::new() };
//!
//! let mut value = vi.ctrl.read();
//! value.set_depth(ColorDepth::BPP32);
//! vi.ctrl.write(value);
//! ```
//! Just like the above example, but using the modify method:
//! ```
//! use n64_pac::vi::{ColorDepth, VideoInterface};
//!
//! let vi = unsafe { VideoInterface::new() };
//!
//! vi.ctrl.modify(|value| value.with_depth(ColorDepth::BPP32));
//! ```

#![no_std]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use crate::ai::AudioInterface;
use crate::cp0::Cp0;
use crate::cp1::Cp1;
use crate::mi::MipsInterface;
use crate::pi::PeripheralInterface;
use crate::si::SerialInterface;
use crate::vi::VideoInterface;

macro_rules! regfn_ro {
    ($block:ident, $reg:ident, $reg_name:expr, $datatype:ident) => {
        #[doc = concat!("Creates a temporary pointer to the [`", stringify!($block), "`], and reads data from its ", stringify!($reg_name), " register.")]
        #[inline(always)]
        pub fn $reg() -> $datatype {
            unsafe { $block::new().$reg.read() }
        }
    };
}
macro_rules! regfn_wo {
    ($block:ident, $reg:ident, $reg_name:expr, $datatype:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a temporary pointer to the [`", stringify!($block), "`], and writes data to its ", stringify!($reg_name), " register.")]
            #[inline(always)]
            pub unsafe fn [<set_ $reg>](data: $datatype) {
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
            pub unsafe fn [<modify_ $reg>]<F: FnOnce($datatype) -> $datatype>(func: F) {
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
            pub unsafe fn [<set_ $reg>](data: [<$uniontype Write>]) {
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

macro_rules! cpxmethod_ro {
    ($reg:ident, $datatype:ident) => {
        pub fn $reg(&self) -> $datatype {
            $reg()
        }
    }
}
macro_rules! cpxmethod_wo {
    ($reg:ident, $datatype:ident) => {
        paste::paste! {
            pub fn [<set_ $reg>](&self, data: $datatype) {
                unsafe { [<set_ $reg>](data); }
            }
        }
    }
}
macro_rules! cpxmethod_rw {
    ($reg:ident, $datatype:ident) => {
        cpxmethod_ro!($reg, $datatype);
        cpxmethod_wo!($reg, $datatype);

        paste::paste! {
            pub fn [<modify_ $reg>]<F: FnOnce($datatype) -> $datatype>(&self, func: F) {
                unsafe { [<set_ $reg>](func($reg())); }
            }
        }
    }
}

macro_rules! derive_tofrom_primitive {
    ($kind:ident, $prim:ident) => {
        impl From<$prim> for $kind {
            fn from(value: $prim) -> Self {
                Self(value)
            }
        }
        impl From<$kind> for $prim {
            fn from(value: $kind) -> Self {
                value.0
            }
        }
    }
}

pub mod ai;
pub mod cp0;
pub mod cp1;
pub mod mi;
pub mod pi;
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
        let ptr = &self.0 as *const T as *mut T;
        unsafe { ptr.write_volatile(func(ptr.read_volatile())); }
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

static mut HARDWARE_TAKEN: bool = false;

/// Represents all hardware abstractions.
///
/// For safe use of hardware, this type follows a singleton pattern. Only one instance of `Hardware`
/// can safely exist at a time. If this is too restrictive for an application, then multiple instances
/// can be created using [`Hardware::steal()`].
///
/// Creating multiple instances of this abstraction, or any other abstraction type, could result in
/// data races when interrupts are enabled, or if using async Rust.
pub struct Hardware {
    pub cp0: Cp0,
    pub cp1: Cp1,
    pub mi: MipsInterface,
    pub vi: VideoInterface,
    pub ai: AudioInterface,
    pub pi: PeripheralInterface,
    //pub ri: RdramInterface,
    pub si: SerialInterface,
}
impl Hardware {
    /// Attempts to take a singleton instance of `Hardware` and return it.
    ///
    /// If `take()` has already been called, `None` will be returned.
    ///
    /// If you need multiple instances, consider using [`Hardware::steal()`].
    #[inline]
    pub fn take() -> Option<Self> {
        if unsafe { HARDWARE_TAKEN } {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    /// Bypasses the singleton pattern, providing a new abstraction instance of the available hardware.
    ///
    /// # Safety
    /// If interrupts are enabled, and the same hardware is being modified in both regular code and
    /// the interrupt handler, a data race might occur. Such that, regular code may have read from
    /// a register and modified it, but then an interrupt occurs before the write back happens.
    ///
    /// If the interrupt handler writes to the same register, once the interrupt handler finishes, the
    /// regular code will overwrite whatever value the handler had written.
    ///
    /// When interrupts are not used, or if care is taken to which registers are written to inside and
    /// outside interrupt handlers, then this method _should_ be safe.
    #[inline]
    pub unsafe fn steal() -> Self {
        HARDWARE_TAKEN = true;

        Self {
            cp0: Cp0::new(),
            cp1: Cp1::new(),
            mi: MipsInterface::new(),
            vi: VideoInterface::new(),
            ai: AudioInterface::new(),
            pi: PeripheralInterface::new(),
            //ri: RdramInterface::new(),
            si: SerialInterface::new(),
        }
    }
}