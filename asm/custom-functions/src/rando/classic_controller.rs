use core::{
    ffi::c_void,
    ptr::{addr_of, addr_of_mut},
};

use crate::{
    game::pad::{
        convertDpdPosToScreenPos__4dPadFR7mVec2_cR7mVec2_c, g_currentCore__4mPad, KPADReadEx,
        KPADStatus, WpadButton, WpadButtonCl, WpadDevType, LINK_ITEM_SELECT_ANY_SELECTED,
    },
    system::math::{atan2__Q23EGG7Math_f_Fff, sRadToAng__4mAng, sqrt__Q23EGG7Math_f_Ff},
};

macro_rules! map_btn {
    ($status:ident, $cl_button:ident, $core_button:ident) => {{
        if (*$status).ex_status.cl.trig & WpadButtonCl::$cl_button as u32 != 0 {
            (*$status).trig |= WpadButton::$core_button as u32;
        }

        if (*$status).ex_status.cl.hold & WpadButtonCl::$cl_button as u32 != 0 {
            (*$status).hold |= WpadButton::$core_button as u32;
        }

        if (*$status).ex_status.cl.release & WpadButtonCl::$cl_button as u32 != 0 {
            (*$status).release |= WpadButton::$core_button as u32;
        }
    }};
}

#[no_mangle]
extern "C" fn kpad_read_ex_wrapper(
    chan: i32,
    status: *mut KPADStatus,
    buf_size: i32,
    kpad_result: *mut i32,
) -> i32 {
    let read_length = unsafe { KPADReadEx(chan, status, buf_size, kpad_result) };

    if read_length > 0 {
        for i in 0..(read_length as usize) {
            let this_status = status.wrapping_add(i);
            unsafe {
                if (*this_status).dev_type == WpadDevType::WPAD_DEV_CLASSIC as u8 {
                    // if a classic controller is attached, remap a bunch of inputs
                    // TODO - mapping two buttons to (A) is nice but the Trig/Release logic
                    // would be wrong since it'll Trig when pressing the second button even if
                    // the first is already pressed
                    // map_btn!(this_status, WPAD_BUTTON_CL_A, WPAD_BUTTON_A);
                    map_btn!(this_status, WPAD_BUTTON_CL_ZR, WPAD_BUTTON_A);

                    map_btn!(this_status, WPAD_BUTTON_CL_Y, WPAD_BUTTON_B);
                    map_btn!(this_status, WPAD_BUTTON_CL_FULL_L, WPAD_BUTTON_FS_Z);
                    map_btn!(this_status, WPAD_BUTTON_CL_ZL, WPAD_BUTTON_FS_C);

                    map_btn!(this_status, WPAD_BUTTON_CL_UP, WPAD_BUTTON_UP);
                    map_btn!(this_status, WPAD_BUTTON_CL_DOWN, WPAD_BUTTON_DOWN);
                    map_btn!(this_status, WPAD_BUTTON_CL_LEFT, WPAD_BUTTON_LEFT);
                    map_btn!(this_status, WPAD_BUTTON_CL_RIGHT, WPAD_BUTTON_RIGHT);

                    map_btn!(this_status, WPAD_BUTTON_CL_HOME, WPAD_BUTTON_HOME);

                    // Save anything that would be overwritten later when we clear sensor values,
                    // since the right stick is actually needed
                    (*this_status).ex_status.rd.rstick = (*this_status).ex_status.cl.rstick;

                    // turns out that the game doesn't really care what type of extension is
                    // actually used; it unconditionally reads the Nunchuk values anyway.
                    // So we can copy over our left stick, but need to make sure to clear sensor
                    // values
                    (*this_status).ex_status.fs.stick = (*this_status).ex_status.cl.lstick;
                    (*this_status).ex_status.fs.acc = [0.0; 3];
                    (*this_status).ex_status.fs.acc_speed = 0.0;
                    (*this_status).ex_status.fs.acc_value = 0.0;
                }
            }
        }
    }

    read_length
}

#[no_mangle]
extern "C" fn get_aiming_stick_dir(_this: *mut c_void, dpd_pos: *mut [f32; 2]) {
    unsafe {
        let mut pos = [0.0; 2];
        if !g_currentCore__4mPad.is_null()
            && (*g_currentCore__4mPad).mCoreStatus[0].dev_type
                == WpadDevType::WPAD_DEV_CLASSIC as u8
        {
            pos = (*g_currentCore__4mPad).mCoreStatus[0].ex_status.rd.rstick;
            pos[1] *= -1.0;
        }

        *dpd_pos = pos;
    }
}

#[link_section = "data"]
static mut TMP_VEC: [f32; 2] = [0.0, 0.0];

#[no_mangle]
extern "C" fn get_item_select_stick_dir() -> *const [f32; 2] {
    unsafe {
        let mut pos = [0.0; 2];
        if !g_currentCore__4mPad.is_null()
            && (*g_currentCore__4mPad).mCoreStatus[0].dev_type
                == WpadDevType::WPAD_DEV_CLASSIC as u8
        {
            pos = (*g_currentCore__4mPad).mCoreStatus[0].ex_status.rd.rstick;
            pos[1] *= -1.0;
        }
        convertDpdPosToScreenPos__4dPadFR7mVec2_cR7mVec2_c(&pos, addr_of_mut!(TMP_VEC));
    }

    addr_of!(TMP_VEC)
}

// in_threshold = 12, out_threshold = 7
#[no_mangle]
extern "C" fn calc_item_select_length_angle(
    angle: *mut i16,
    length: *mut f32,
    in_threshold: *const i16,
    out_threshold: *const i16,
) {
    unsafe {
        let mut pos = [0.0; 2];
        if !g_currentCore__4mPad.is_null()
            && (*g_currentCore__4mPad).mCoreStatus[0].dev_type
                == WpadDevType::WPAD_DEV_CLASSIC as u8
        {
            pos = (*g_currentCore__4mPad).mCoreStatus[0].ex_status.rd.rstick;
        }

        *length = sqrt__Q23EGG7Math_f_Ff(pos[0] * pos[0] + pos[1] * pos[1]);
        *angle = (atan2__Q23EGG7Math_f_Fff(-pos[0], pos[1]) * sRadToAng__4mAng) as i16;
        if LINK_ITEM_SELECT_ANY_SELECTED && *length < 0.10 {
            LINK_ITEM_SELECT_ANY_SELECTED = false;
        } else if !LINK_ITEM_SELECT_ANY_SELECTED && *length >= 0.25 {
            LINK_ITEM_SELECT_ANY_SELECTED = true;
        }
    }
}

#[no_mangle]
extern "C" fn get_beetle_flying_zrot(angle: *mut i16) {
    unsafe {
        let mut rot = 0.0;
        if !g_currentCore__4mPad.is_null()
            && (*g_currentCore__4mPad).mCoreStatus[0].dev_type
                == WpadDevType::WPAD_DEV_CLASSIC as u8
        {
            rot = (*g_currentCore__4mPad).mCoreStatus[0].ex_status.rd.rstick[0];
        }

        *angle = (rot * -16384.0) as i16;
    }
}

#[no_mangle]
extern "C" fn get_beetle_flying_yrot() -> i16 {
    unsafe {
        let mut rot = 0.0;
        if !g_currentCore__4mPad.is_null()
            && (*g_currentCore__4mPad).mCoreStatus[0].dev_type
                == WpadDevType::WPAD_DEV_CLASSIC as u8
        {
            rot = (*g_currentCore__4mPad).mCoreStatus[0].ex_status.rd.rstick[1];
        }

        (rot * 16384.0) as i16
    }
}
