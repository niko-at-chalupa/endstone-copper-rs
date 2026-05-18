/// Should mirror RsPluginVTable in shim.cpp
#[repr(C)]
pub struct RsPluginVTable {
    pub on_load:        Option<unsafe extern "C" fn(*mut ())>,
    pub on_enable:      Option<unsafe extern "C" fn(*mut ())>,
    pub on_disable:     Option<unsafe extern "C" fn(*mut ())>,
    pub on_player_join: Option<unsafe extern "C" fn(*mut (), *mut ())>,
    pub drop:           Option<unsafe extern "C" fn(*mut ())>,
}

/// The metadata struct that mirrors RsPluginMeta in shim.cpp
#[repr(C)]
pub struct RsPluginMeta {
    pub name:        *const std::ffi::c_char,
    pub version:     *const std::ffi::c_char,
    pub description: *const std::ffi::c_char,
    pub author:      *const std::ffi::c_char,
}

// Globals that shim.cpp reads after we fill them in
#[unsafe(no_mangle)]
pub static mut ENDSTONE_RS_VTABLE: RsPluginVTable = RsPluginVTable {
    on_load:        None,
    on_enable:      None,
    on_disable:     None,
    on_player_join: None,
    drop:           None,
};

#[unsafe(no_mangle)]
pub static mut ENDSTONE_RS_META: RsPluginMeta = RsPluginMeta {
    name:        std::ptr::null(),
    version:     std::ptr::null(),
    description: std::ptr::null(),
    author:      std::ptr::null(),
};

#[unsafe(no_mangle)]
pub static mut ENDSTONE_RS_PLUGIN_PTR: *mut () = std::ptr::null_mut();

unsafe extern "C" {
    fn endstone_rs_init_plugin() -> *mut ();
    pub fn endstone_player_send_message(player: *mut (), message: *const std::ffi::c_char);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_endstone_plugin() -> *mut () {
    unsafe {
        endstone_rs_init_plugin()
    }
}