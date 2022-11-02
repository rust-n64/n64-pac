
//TODO: Add remaining memory mapped registers

macro_rules! reg {
    ($name:ident, $base:expr, $offset:expr) => {
        pub const $name: *mut u32 = ($base + $offset) as *mut u32;
    };
}


macro_rules! reg_mi {
    ($name:ident, $offset:expr) => {
        reg!($name, 0xA4300000u32, $offset);
    };
}

reg_mi!(MI_BASE, 0x00);
reg_mi!(MI_MODE, 0x00);
reg_mi!(MI_VERSION, 0x04);
reg_mi!(MI_INTERRUPT, 0x08);
reg_mi!(MI_MASK, 0x0C);


macro_rules! reg_vi {
    ($name:ident, $offset:expr) => {
        reg!($name, 0xA4400000u32, $offset);
    };
}

reg_vi!(VI_BASE, 0x00);
reg_vi!(VI_CTRL, 0x00);
reg_vi!(VI_ORIGIN, 0x04);
reg_vi!(VI_WIDTH, 0x08);
reg_vi!(VI_V_INTR, 0x0C);
reg_vi!(VI_V_CURRENT, 0x10);
reg_vi!(VI_BURST, 0x14);
reg_vi!(VI_V_SYNC, 0x18);
reg_vi!(VI_H_SYNC, 0x1C);
reg_vi!(VI_H_SYNC_LEAP, 0x20);
reg_vi!(VI_H_VIDEO, 0x24);
reg_vi!(VI_V_VIDEO, 0x28);
reg_vi!(VI_V_BURST, 0x2C);
reg_vi!(VI_X_SCALE, 0x30);
reg_vi!(VI_Y_SCALE, 0x34);
reg_vi!(VI_TEST_ADDR, 0x38);
reg_vi!(VI_STAGED_DATA, 0x3C);


macro_rules! reg_si {
    ($name:ident, $offset:expr) => {
        reg!($name, 0xA4800000u32, $offset);
    };
}

reg_si!(SI_BASE, 0x00);
reg_si!(SI_DRAM_ADDR, 0x00);
reg_si!(SI_PIF_AD_RD64B, 0x04);
reg_si!(SI_PIF_AD_WR4B, 0x08);
reg_si!(SI_PIF_AD_WR64B, 0x10);
reg_si!(SI_PIF_AD_RD4B, 0x14);
reg_si!(SI_STATUS, 0x18);