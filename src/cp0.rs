//! CPU - Coprocessor 0

use core::arch::asm;
use core::marker::PhantomData;
use num_enum::{FromPrimitive, IntoPrimitive};
use proc_bitfield::bitfield;

//TODO: Complete rustdocs for all bitfields

macro_rules! cp0fn_ro {
    ($reg:ident, $width:ident, $index:literal, $datatype:ident) => {
        paste::paste! {
            #[doc = concat!("Reads from CP0 register ", stringify!($index), ".")]
            #[inline(always)]
            pub fn $reg() -> $datatype {
                [<read_ $width>]::<$index>().into()
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
                [<write_ $width>]::<$index>(data.into());
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

/// A zero-sized struct for accessing CP0 registers via methods.
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
    
    cpxmethod_rw!(index, IndexReg);
    cpxmethod_rw!(random, RandomReg);
    cpxmethod_rw!(entrylo0, EntryLoReg);
    cpxmethod_rw!(entrylo1, EntryLoReg);
    cpxmethod_rw!(context, ContextReg);
    cpxmethod_rw!(pagemask, PageMaskReg);
    cpxmethod_rw!(wired, WiredReg);
    cpxmethod_ro!(badvaddr, BadVAddrReg);
    cpxmethod_rw!(count, u32);
    cpxmethod_rw!(entryhi, EntryHiReg);
    cpxmethod_rw!(compare, u32);
    cpxmethod_rw!(status, StatusReg);
    cpxmethod_rw!(cause, CauseReg);
    cpxmethod_rw!(exception_pc, ExceptionPcReg);
    cpxmethod_ro!(processor_revision_id, ProcessorRevisionIdReg);
    cpxmethod_rw!(config, ConfigReg);
    cpxmethod_rw!(load_linked_address, u32);
    cpxmethod_rw!(watchlo, WatchLoReg);
    cpxmethod_rw!(watchhi, WatchHiReg);
    cpxmethod_rw!(xcontext, XContextReg);
    cpxmethod_rw!(parity_error, ParityErrorReg);
    cpxmethod_rw!(taglo, TagLoReg);
    cpxmethod_rw!(error_exception_pc, ErrorExceptionPcReg);
}

cp0fn_rw!(index, u32, 0, IndexReg);
cp0fn_rw!(random, u32, 1, RandomReg);
cp0fn_rw!(entrylo0, u32, 2, EntryLoReg);
cp0fn_rw!(entrylo1, u32, 3, EntryLoReg);
cp0fn_rw!(context, u64, 4, ContextReg);
cp0fn_rw!(pagemask, u32, 5, PageMaskReg);
cp0fn_rw!(wired, u32, 6, WiredReg);
cp0fn_ro!(badvaddr, u64, 8, BadVAddrReg);
cp0fn_rw!(count, u32, 9, u32);
cp0fn_rw!(entryhi, u64, 10, EntryHiReg);
cp0fn_rw!(compare, u32, 11, u32);
cp0fn_rw!(status, u32, 12, StatusReg);
cp0fn_rw!(cause, u32, 13, CauseReg);
cp0fn_rw!(exception_pc, u64, 14, ExceptionPcReg);
cp0fn_ro!(processor_revision_id, u32, 15, ProcessorRevisionIdReg);
cp0fn_rw!(config, u32, 16, ConfigReg);
cp0fn_rw!(load_linked_address, u32, 17, u32);
cp0fn_rw!(watchlo, u32, 18, WatchLoReg);
cp0fn_rw!(watchhi, u32, 19, WatchHiReg);
cp0fn_rw!(xcontext, u64, 20, XContextReg);
cp0fn_rw!(parity_error, u32, 26, ParityErrorReg);
cp0fn_rw!(taglo, u32, 28, TagLoReg);
cp0fn_rw!(error_exception_pc, u64, 30, ErrorExceptionPcReg);


bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct IndexReg(pub u32): Debug {
        pub index: u8 @ 0..=5,
        pub probe: bool @ 31,
    }
}
derive_tofrom_primitive!(IndexReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RandomReg(pub u32): Debug {
        pub random: u8 [ro] @ 0..=5,
    }
}
derive_tofrom_primitive!(RandomReg, u32);

#[derive(IntoPrimitive, FromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CacheAlgorithm {
    Uncached = 0b010,
    #[default]
    Cached = 0b011,
}

bitfield! {
    /// Contains the page frame number and other configuration bits for a TLB entry.
    /// 
    /// EntryLo0 and EntryLo1 registers use the same format, thus this type can be used for both registers.
    /// 
    /// EntryLo0 is used for even virtual pages, EntryLo1 for odd virtual pages.
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct EntryLoReg(pub u32): Debug {
        pub global: bool @ 0,
        pub valid: bool @ 1,
        pub dirty: bool @ 2,
        pub cache_algorithm: u8 [CacheAlgorithm] @ 3..=5,
        pub page_frame_number: u32 @ 6..=29,
    }
}
derive_tofrom_primitive!(EntryLoReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ContextReg(pub u64): Debug {
        /// Page number of virtual address whose translation is invalid, divided by 2
        pub bad_vpn2: u32 @ 4..=22,
        
        /// Base address of the page table entry (32-bit mode)
        pub pte_base_u32: u32 @ 23..=31,
        
        /// Base address of the page table entry (64-bit mode)
        pub pte_base_u64: u64 @ 23..=63,
    }
}
derive_tofrom_primitive!(ContextReg, u64);

#[derive(IntoPrimitive, FromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum PageSize {
    KB4 = 0x000,
    KB16 = 0x003,
    KB64 = 0x00F,
    KB256 = 0x03F,
    MB1 = 0x0FF,
    MB4 = 0x3FF,
    MB16 = 0xFFF,
    #[default]
    Undefined,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct PageMaskReg(pub u32): Debug {
        pub mask: u16 [PageSize] @ 13..=24,
    }
}
derive_tofrom_primitive!(PageMaskReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct WiredReg(pub u32): Debug {
        pub wired: u8 @ 0..=5,
    }
}
derive_tofrom_primitive!(WiredReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct BadVAddrReg(pub u64): Debug {
        /// Most recently translated vitual address that had an invalid translation or an addressing error (32-bit mode)
        pub badvaddr_u32: u32 [ro] @ 0..=31,
        
        /// Most recently translated vitual address that had an invalid translation or an addressing error (64-bit mode)
        pub badvaddr_u64: u64 [ro] @ 0..=63,
    }
}
derive_tofrom_primitive!(BadVAddrReg, u64);

#[derive(IntoPrimitive, FromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum VAddrRegion {
    User = 0,
    Supervisor = 1,
    #[default]
    Unknown = 2,
    Kernel = 3,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct EntryHiReg(pub u64): Debug {
        /// Address space identifier
        pub asid: u8 @ 0..=7,
        
        /// Virtual page number divided by 2 (32-bit mode)
        pub vpn2_u32: u32 @ 13..=31,
        
        /// Virtual page number divided by 2 (64-bit mode)
        pub vpn2_u64: u32 @ 13..=39,
        
        /// Reserved, undefined on read (64-bit mode)
        pub fill: u32 @ 40..=61,
        
        /// Region used to match bits \[63:62\] of the virtual address (64-bit mode)
        pub region: u8 [VAddrRegion] @ 62..=63,
    }
}
derive_tofrom_primitive!(EntryHiReg, u64);

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
derive_tofrom_primitive!(StatusReg, u32);

#[derive(IntoPrimitive, FromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ExceptionCode {
    Interrupt = 0,
    TlbModification = 1,
    TlbMissOnLoad = 2,
    TlbMissOnStore = 3,
    AddressErrorOnLoad = 4,
    AddressErrorOnStore = 5,
    InstructionBusError = 6,
    DataBusError = 7,
    Syscall = 8,
    Breakpoint = 9,
    ReservedInstruction = 10,
    CoprocessorUnusable = 11,
    ArithmeticOverflow = 12,
    Trap = 13,
    // Reserved = 14
    FloatingPoint = 15,
    // Reserved = 16-22
    Watch = 23,
    // Reserved = 24-31
    #[default]
    Reserved,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct CauseReg(pub u32): Debug {
        pub exception_code: u8 [ExceptionCode, ro] @ 2..=6,
        
        /// Software interrupt 0, set true to trigger interrupt
        pub ip0: bool @ 8,
        /// Software interrupt 1, set true to trigger interrupt
        pub ip1: bool @ 9,
        
        /// External interrupt `/INT0` (connected to RCP for IO Interface interrupts)
        pub ip2: bool [ro] @ 10,
        /// External interrupt `/INT1` (available on cartridge port pin 44)
        pub ip3: bool [ro] @ 11,
        /// External interrupt `/INT2` (aka PRE_NMI, triggers when PIF detects reset button was pressed)
        pub ip4: bool [ro] @ 12,
        /// External interrupt `/INT3` (pulled high, can't be triggered without hardware modifications)
        pub ip5: bool [ro] @ 13,
        /// External interrupt `/INT4` (pulled high, can't be triggered without hardware modifications)
        pub ip6: bool [ro] @ 14,
        /// Timer interrupt
        pub ip7: bool [ro] @ 15,
        
        /// Coprocessor unit number referenced when a Coprocessor Unusable exception has occurred. Otherwise undefined.
        pub ce: u8 [ro] @ 28..=29,
        
        /// Indicates whether the last exception occured in a branch delay slot.
        /// 
        /// - 0 = normal
        /// - 1 = delay slot
        pub branch_delay: bool [ro] @ 31,
    }
}
derive_tofrom_primitive!(CauseReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ExceptionPcReg(pub u64): Debug {
        /// The 32-bit address at which processing resumes after an exception/interrupt has been serviced. (32-bit mode)
        pub epc_u32: u32 @ 0..=31,
        
        /// The 64-bit address at which processing resumes after an exception/interrupt has been serviced. (64-bit mode)
        pub epc_u64: u64 @ 0..=63,
    }
}
derive_tofrom_primitive!(ExceptionPcReg, u64);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ProcessorRevisionIdReg(pub u32): Debug {
        /// Processor revision number
        pub revision: u8 [ro] @ 0..=7,
        /// Likely is 0x0B for all VR4300 series CPUs
        pub processor_id: u8 [ro] @ 8..=15,
    }
}
derive_tofrom_primitive!(ProcessorRevisionIdReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ConfigReg(pub u32): Debug {
        /// Coherency algorithm for kernel segment 0 (kseg0)
        pub k0: u8 [CacheAlgorithm] @ 0..=2,
        
        /// Reserved, but can be read/written by software
        pub cu: bool @ 3,
        
        /// Sets memory endianness
        /// 
        /// - 0 = little endian
        /// - 1 = big endian (default on cold reset)
        pub be: bool @ 15,
        
        /// Sets writeback data pattern for the SysAD bus
        /// 
        /// - 0 = D (default on cold reset)
        /// - 6 = DxxDxx (2 doublewords / 6 cycles)
        /// - Others = Reserved/Unknown
        pub ep: u8 @ 24..=27,
        
        /// Operating frequency ratio
        /// 
        /// The value corresponds to the frequency ratio set by the DivMode pins of the CPU hardware.
        pub ec: u8 [ro] @ 28..=30,
    }
}
derive_tofrom_primitive!(ConfigReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct WatchLoReg(pub u32): Debug {
        /// If true, trigger an exception when a store instruction is executed.
        pub w: bool @ 0,
        /// If true, trigger an exception when a load instruction is executed.
        pub r: bool @ 1,
        
        /// Bits \[31:3\] of the physical address to watch for.
        /// 
        /// Bits \[35:32\] are set in [`WatchHiReg`].
        pub paddr0: u32 @ 3..=31,
    }
}
derive_tofrom_primitive!(WatchLoReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct WatchHiReg(pub u32): Debug {
        /// Bits \[35:32\] of the physical address to watch for.
        /// 
        /// Bits \[31:3\] are set in [`WatchLoReg`].
        /// 
        /// These bits are only for compatibility with other CPU models, **and serve no purpose in the
        /// VR4300 (N64's CPU).** However, they are still readable/writable by software. 
        pub paddr1: u32 @ 0..=3,
    }
}
derive_tofrom_primitive!(WatchHiReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct XContextReg(pub u64): Debug {
        pub badvpn2: u32 @ 4..=30,
        pub region: u8 [VAddrRegion] @ 31..=32,
        pub ptebase: u32 @ 33..=63,
    }
}
derive_tofrom_primitive!(XContextReg, u64);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ParityErrorReg(pub u32): Debug {
        pub diagnostic: u8 @ 0..=7,
    }
}
derive_tofrom_primitive!(ParityErrorReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct TagLoReg(pub u32): Debug {
        /// Specifies the primary cache state
        /// 
        /// Data cache
        /// - 0 = Invalid
        /// - 3 = Valid
        /// 
        /// Instruction cache
        /// - 0 = Invalid
        /// - 2 = Valid
        /// 
        /// Others = Undefined
        pub pstate: u8 @ 6..=7,
        
        /// Physical address bits \[31:12\]
        pub ptaglo: u32 @ 8..=27,
    }
}
derive_tofrom_primitive!(TagLoReg, u32);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ErrorExceptionPcReg(pub u64): Debug {
        /// The 32-bit program counter address on cold reset, soft reset, or NMI exception. (32-bit mode)
        pub epc_u32: u32 @ 0..=31,
        
        /// The 64-bit program counter address on cold reset, soft reset, or NMI exception. (64-bit mode)
        pub epc_u64: u64 @ 0..=63,
    }
}
derive_tofrom_primitive!(ErrorExceptionPcReg, u64);


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