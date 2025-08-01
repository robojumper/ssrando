#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KPADEXStatusCL {
    pub hold:     u32,      // at 0x0
    pub trig:     u32,      // at 0x4
    pub release:  u32,      // at 0x8
    pub lstick:   [f32; 2], // at 0xC
    pub rstick:   [f32; 2], // at 0x14
    pub ltrigger: f32,      // at 0x1C
    pub rtrigger: f32,      // at 0x20
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KPADEXStatusFS {
    pub stick:     [f32; 2], // at 0x0
    pub acc:       [f32; 3], // at 0x8
    pub acc_value: f32,      // at 0x14
    pub acc_speed: f32,      // at 0x18
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KPADEXStatusBL {
    _bl: [u8; 0x50],
}

#[repr(C)]
#[derive(Clone, Copy)]
/// We need to save some values to somewhere where they won't bother the
/// game code. Thankfully the Balance Board (BL) is 0x50 bytes large,
/// which gives us some extra space to work with, as long as
/// `KPADEXStatusRando` is 0x50 bytes large or less
pub struct KPADEXStatusRando {
    hidden_fs:  KPADEXStatusFS, // at 0x0
    pub rstick: [f32; 2],       // at 0x1C
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KPADEXStatus {
    pub cl: KPADEXStatusCL,    // at 0x00
    pub fs: KPADEXStatusFS,    // at 0x00
    pub bl: KPADEXStatusBL,    // at 0x00
    pub rd: KPADEXStatusRando, // at 0x00
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KPADStatus {
    pub hold:         u32,          // at 0x0
    pub trig:         u32,          // at 0x4
    pub release:      u32,          // at 0x8
    pub acc:          [f32; 3],     // at 0xC
    pub acc_value:    f32,          // at 0x18
    pub acc_speed:    f32,          // at 0x1C
    pub pos:          [f32; 2],     // at 0x20
    pub vec:          [f32; 2],     // at 0x28
    pub speed:        f32,          // at 0x30
    pub horizon:      [f32; 2],     // at 0x34
    pub hori_vec:     [f32; 2],     // at 0x3C
    pub hori_speed:   f32,          // at 0x44
    pub dist:         f32,          // at 0x48
    pub dist_vec:     f32,          // at 0x4C
    pub dist_speed:   f32,          // at 0x50
    pub acc_vertical: [f32; 2],     // at 0x54
    pub dev_type:     u8,           // at 0x5C
    pub wpad_err:     i8,           // at 0x5D
    pub dpd_valid_fg: i8,           // at 0x5E
    pub data_format:  u8,           // at 0x5F
    pub ex_status:    KPADEXStatus, // at 0x60
    pub mpls_rot:     [f32; 3],     // at 0xB0 // made up
    pub field_0xBC:   [f32; 3],     // at 0xBC // made up
    pub mpls_basis_x: [f32; 3],     // at 0xC8 // made up
    pub mpls_basis_y: [f32; 3],     // at 0xD4 // made up
    pub mpls_basis_z: [f32; 3],     // at 0xE0 // made up
    pub field_0xEC:   u32,
}

#[repr(u32)]
pub enum WpadButton {
    WPAD_BUTTON_LEFT  = 1 << 0,
    WPAD_BUTTON_RIGHT = 1 << 1,
    WPAD_BUTTON_DOWN  = 1 << 2,
    WPAD_BUTTON_UP    = 1 << 3,
    WPAD_BUTTON_PLUS  = 1 << 4,
    WPAD_BUTTON_2     = 1 << 8,
    WPAD_BUTTON_1     = 1 << 9,
    WPAD_BUTTON_B     = 1 << 10,
    WPAD_BUTTON_A     = 1 << 11,
    WPAD_BUTTON_MINUS = 1 << 12,
    WPAD_BUTTON_FS_Z  = 1 << 13,
    WPAD_BUTTON_FS_C  = 1 << 14,
    WPAD_BUTTON_HOME  = 1 << 15,
}

#[repr(u32)]
pub enum WpadButtonCl {
    WPAD_BUTTON_CL_UP     = 1 << 0,
    WPAD_BUTTON_CL_LEFT   = 1 << 1,
    WPAD_BUTTON_CL_ZR     = 1 << 2,
    WPAD_BUTTON_CL_X      = 1 << 3,
    WPAD_BUTTON_CL_A      = 1 << 4,
    WPAD_BUTTON_CL_Y      = 1 << 5,
    WPAD_BUTTON_CL_B      = 1 << 6,
    WPAD_BUTTON_CL_ZL     = 1 << 7,
    WPAD_BUTTON_CL_FULL_R = 1 << 9,
    WPAD_BUTTON_CL_PLUS   = 1 << 10,
    WPAD_BUTTON_CL_HOME   = 1 << 11,
    WPAD_BUTTON_CL_MINUS  = 1 << 12,
    WPAD_BUTTON_CL_FULL_L = 1 << 13,
    WPAD_BUTTON_CL_DOWN   = 1 << 14,
    WPAD_BUTTON_CL_RIGHT  = 1 << 15,
}

#[repr(u8)]
pub enum WpadDevType {
    WPAD_DEV_CLASSIC = 2,
}

#[repr(C)]
pub struct EggCoreController {
    _pad:            [u8; 0x18],       // at 0x00
    pub mCoreStatus: [KPADStatus; 16], // at 0x18
}

extern "C" {
    pub fn KPADReadEx(
        chan: i32,
        status: *mut KPADStatus,
        bufSize: i32,
        kpad_result: *mut i32,
    ) -> i32;

    pub fn convertDpdPosToScreenPos__4dPadFR7mVec2_cR7mVec2_c(
        input: *const [f32; 2],
        output: *mut [f32; 2],
    );

    pub static g_currentCore__4mPad: *mut EggCoreController;
    pub static mut LINK_ITEM_SELECT_ANY_SELECTED: bool;
}
