use core::{
    f32::consts::PI,
    ffi::c_void,
    ptr::{addr_of, addr_of_mut},
};

use crate::{
    game::pad::{
        convertDpdPosToScreenPos__4dPadFR7mVec2_cR7mVec2_c, g_currentCore__4mPad, KPADReadEx,
        KPADStatus, WpadButton, WpadButtonCl, WpadDevType, LINK_ITEM_SELECT_ANY_SELECTED,
    },
    system::{
        hbm,
        math::{
            atan2__Q23EGG7Math_f_Fff, cos__Q23EGG7Math_f_Ff, sRadToAng__4mAng,
            sin__Q23EGG7Math_f_Ff, sqrt__Q23EGG7Math_f_Ff,
        },
    },
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

#[link_section = "data"]
static mut RSTICK_HISTORY: [[f32; 2]; 120] = [[0.0; 2]; 120];

#[link_section = "data"]
static mut LAST_HBM_LSTICK: [f32; 2] = [0.0; 2];

#[no_mangle]
extern "C" fn kpad_read_ex_wrapper(
    chan: i32,
    status: *mut KPADStatus,
    buf_size: i32,
    kpad_result: *mut i32,
) -> i32 {
    let read_length = unsafe { KPADReadEx(chan, status, buf_size, kpad_result) };

    // On the Home Button Menu, skip our input remapping and move the cursor
    if hbm::is_hbm_active() {
        if read_length > 0 {
            unsafe {
                if (*status).dev_type == WpadDevType::WPAD_DEV_CLASSIC as u8 {
                    LAST_HBM_LSTICK = (*status).ex_status.cl.lstick;
                    LAST_HBM_LSTICK[0] /= 100.0;
                    LAST_HBM_LSTICK[1] /= -100.0;
                }
            }
        }
        hbm::delta_cursor_position(unsafe { LAST_HBM_LSTICK });
        return read_length;
    }

    if read_length > 0 {
        // Shift values back, like in d_pad
        for i in ((read_length as usize)..120).rev() {
            unsafe {
                RSTICK_HISTORY[i] = RSTICK_HISTORY[i - (read_length as usize)];
            }
        }

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

                    map_btn!(this_status, WPAD_BUTTON_CL_PLUS, WPAD_BUTTON_1);
                    map_btn!(this_status, WPAD_BUTTON_CL_MINUS, WPAD_BUTTON_2);

                    // Save anything that would be overwritten later when we clear sensor values,
                    // since the right stick is actually needed
                    (*this_status).ex_status.rd.rstick = (*this_status).ex_status.cl.rstick;

                    RSTICK_HISTORY[i] = (*this_status).ex_status.cl.rstick;

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

#[no_mangle]
extern "C" fn get_sword_pointing_direction(_this: *mut c_void, dir: *mut [f32; 3]) -> bool {
    unsafe {
        let mut pos = [0.0; 2];
        if !g_currentCore__4mPad.is_null()
            && (*g_currentCore__4mPad).mCoreStatus[0].dev_type
                == WpadDevType::WPAD_DEV_CLASSIC as u8
        {
            pos = (*g_currentCore__4mPad).mCoreStatus[0].ex_status.rd.rstick;
        }

        // transform [-1.0; 1.0]^2 into [-pi/2; pi/2]^2
        pos[0] *= PI / 2.0;
        pos[1] *= PI / 2.0;

        // left = 1, 0, 0
        // up = 0, 1, 0
        // front = 0, 0, 1

        // a bit of a weird way of turning spherical into cartesian coordinates
        // since our angles use a different convention
        (*dir)[0] = -sin__Q23EGG7Math_f_Ff(pos[0]) * cos__Q23EGG7Math_f_Ff(pos[1]);
        (*dir)[1] = sin__Q23EGG7Math_f_Ff(pos[1]);
        (*dir)[2] = cos__Q23EGG7Math_f_Ff(pos[0]) * cos__Q23EGG7Math_f_Ff(pos[1]);
    }

    // TODO not sure what this return value does
    false
}

fn square_mag_2(vec2: &[f32; 2]) -> f32 {
    vec2[0] * vec2[0] + vec2[1] * vec2[1]
}

fn dot_2(a: &[f32; 2], b: &[f32; 2]) -> f32 {
    a[0] * b[0] + a[1] * b[1]
}

#[no_mangle]
extern "C" fn calc_swing_direction(_this: *mut c_void, dir: *mut [f32; 3]) {
    unsafe {
        let mut pos = [0.0; 2];
        let mut pos_x_samples_ago = [0.0; 2];
        if !g_currentCore__4mPad.is_null()
            && (*g_currentCore__4mPad).mCoreStatus[0].dev_type
                == WpadDevType::WPAD_DEV_CLASSIC as u8
        {
            pos = RSTICK_HISTORY[0];
            pos_x_samples_ago = RSTICK_HISTORY[6];
        }

        let this_mag = square_mag_2(&pos);
        if
        // Make sure our swing is significant
        this_mag > 0.8
            && (
                // and we either change stick direction
                dot_2(&pos, &pos_x_samples_ago) <= 0.0
                // or swing outwards, not inwards
                || this_mag > square_mag_2(&pos_x_samples_ago)
            )
        {
            (*dir)[0] = (pos[1] - pos_x_samples_ago[1]) * -10.0;
            (*dir)[1] = (pos[0] - pos_x_samples_ago[0]) * -10.0;
        } else {
            (*dir)[0] = 0.0;
            (*dir)[1] = 0.0;
        }

        (*dir)[2] = 0.0;
    }
}
