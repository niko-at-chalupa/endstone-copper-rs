/// Should mirror RsPluginVTable in shim.cpp
#[repr(C)]
pub struct RsPluginVTable {
    pub on_load: Option<unsafe extern "C" fn(*mut (), *const Server)>,
    pub on_enable: Option<unsafe extern "C" fn(*mut (), *const Server)>,
    pub on_disable: Option<unsafe extern "C" fn(*mut (), *const Server)>,
    pub drop: Option<unsafe extern "C" fn(*mut ())>,
}

/// The metadata struct that mirrors RsPluginMeta in shim.cpp
#[repr(C)]
pub struct RsPluginMeta {
    pub name: *const std::ffi::c_char,
    pub version: *const std::ffi::c_char,
    pub description: *const std::ffi::c_char,
    pub author: *const std::ffi::c_char,
    pub website: *const std::ffi::c_char,
    pub prefix: *const std::ffi::c_char,
}

// Globals that shim.cpp reads after we fill them in
#[unsafe(no_mangle)]
pub static mut ENDSTONE_RS_VTABLE: RsPluginVTable = RsPluginVTable {
    on_load: None,
    on_enable: None,
    on_disable: None,
    drop: None,
};

#[unsafe(no_mangle)]
pub static mut ENDSTONE_RS_META: RsPluginMeta = RsPluginMeta {
    name: std::ptr::null(),
    version: std::ptr::null(),
    description: std::ptr::null(),
    author: std::ptr::null(),
    website: std::ptr::null(),
    prefix: std::ptr::null(),
};

#[unsafe(no_mangle)]
pub static mut ENDSTONE_RS_PLUGIN_PTR: *mut () = std::ptr::null_mut();

unsafe extern "C" {
    fn endstone_rs_init_plugin() -> *mut ();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_endstone_plugin() -> *mut () {
    unsafe { endstone_rs_init_plugin() }
}

#[repr(C)]
pub struct Server {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Server {
    type Id = cxx::type_id!("endstone::Server");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct Logger {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Logger {
    type Id = cxx::type_id!("endstone::Logger");
    type Kind = cxx::kind::Opaque;
}

#[cxx::bridge(namespace = "endstone")]
mod logger_ffi {
    unsafe extern "C++" {
        include!("endstone/logger.h");
        include!("endstone/server.h");

        type Logger = crate::bridge::Logger;
        fn info(self: &Logger, message: &CxxString);
        fn warning(self: &Logger, message: &CxxString);
        fn error(self: &Logger, message: &CxxString);

        type Server = crate::bridge::Server;
        #[rust_name = "get_name"]
        fn getName(self: &Server) -> String;
        #[rust_name = "get_version"]
        fn getVersion(self: &Server) -> String;
        #[rust_name = "get_logger"]
        fn getLogger(self: &Server) -> &Logger;
    }
}