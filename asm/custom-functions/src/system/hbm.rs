use core::ffi::c_void;

extern "C" {
    static s_pInstance__Q24dHbm8Manage_c: *mut c_void;
}

pub fn is_hbm_active() -> bool {
    unsafe {
        if s_pInstance__Q24dHbm8Manage_c.is_null() {
            return false;
        }

        s_pInstance__Q24dHbm8Manage_c
            .cast::<i32>()
            .byte_offset(0x210)
            .read()
            == 2
    }
}

pub fn delta_cursor_position(delta: [f32; 2]) {
    unsafe {
        if s_pInstance__Q24dHbm8Manage_c.is_null() {
            return;
        }

        let vec: *mut [f32; 2] = s_pInstance__Q24dHbm8Manage_c
            .cast::<[f32; 2]>()
            .byte_offset(0xE4);

        (*vec)[0] += delta[0];
        (*vec)[1] += delta[1];

        (*vec)[0] = (*vec)[0].clamp(-1.0, 1.0);
        (*vec)[1] = (*vec)[1].clamp(-1.0, 1.0);
    }
}
