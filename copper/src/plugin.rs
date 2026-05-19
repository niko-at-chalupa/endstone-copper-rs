use std::ffi::CString;
use crate::ffi;

/// Trait plugins should implement
pub trait Plugin: Send + 'static {
    /// Called after a plugin is loaded but before it has been enabled.
    fn on_load(&mut self) {}

    /// Called when this plugin is enabled.
    fn on_enable(&mut self) {}

    /// Called when this plugin is disabled.
    fn on_disable(&mut self) {}
}

/// Represents a player.
pub struct Player {
    ptr: *mut (),
}

impl Player {
    /// Sends this player a message
    pub fn send_message(&self, message: &str) {
        let c_msg = CString::new(message).unwrap();
        unsafe {
            ffi::endstone_player_send_message(self.ptr, c_msg.as_ptr());
        }
    }
}

/// Metadata about your plugin
pub struct PluginMeta {
    pub name:        &'static str,
    pub version:     &'static str,
    pub description: Option<&'static str>,
    pub author:      Option<&'static str>,
}

pub unsafe extern "C" fn trampoline_on_load<T: Plugin>(ptr: *mut ()) {
    unsafe {
        let plugin = &mut *(ptr as *mut T);
        plugin.on_enable();
    }
}

pub unsafe extern "C" fn trampoline_on_enable<T: Plugin>(ptr: *mut ()) {
    unsafe {
        let plugin = &mut *(ptr as *mut T);
        plugin.on_enable();
    }
}

pub unsafe extern "C" fn trampoline_on_disable<T: Plugin>(ptr: *mut ()) {
    unsafe {
        let plugin = &mut *(ptr as *mut T);
        plugin.on_disable();
    }
}   

pub unsafe extern "C" fn trampoline_drop<T: Plugin>(ptr: *mut ()) {
    unsafe {
        drop(Box::from_raw(ptr as *mut T));
    }
}

/// Called by the proc macro's generated `endstone_rs_init()`.
/// Sets up the globals shim.cpp reads.
///
/// # Safety
/// Must only be called once, from `endstone_rs_init()`, before the C++
/// shim reads any of the globals.
pub fn register_plugin<T: Plugin>(plugin: T, meta: PluginMeta) {
    let boxed = Box::new(plugin);
    let raw = Box::into_raw(boxed) as *mut ();

    unsafe {
        ffi::ENDSTONE_RS_PLUGIN_PTR = raw;

        ffi::ENDSTONE_RS_VTABLE = ffi::RsPluginVTable {
            on_load:        Some(trampoline_on_load::<T>),
            on_enable:      Some(trampoline_on_enable::<T>),
            on_disable:     Some(trampoline_on_disable::<T>),
            drop:           Some(trampoline_drop::<T>),
        };

        // We leak the CStrings intentionally — they live for the process lifetime.
        // (Could also be static strs with a wrapper, but this is simpler for now.)
        let name    = CString::new(meta.name).unwrap();
        let version = CString::new(meta.version).unwrap();

        ffi::ENDSTONE_RS_META = ffi::RsPluginMeta {
            name:        name.into_raw(),
            version:     version.into_raw(),
            description: meta.description
                .map(|s| CString::new(s).unwrap().into_raw() as *const _)
                .unwrap_or(std::ptr::null()),
            author:      meta.author
                .map(|s| CString::new(s).unwrap().into_raw() as *const _)
                .unwrap_or(std::ptr::null()),
        };
    }
}