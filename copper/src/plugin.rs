use std::ffi::CString;
use crate::bridge;

/// Context passed to plugin methods
pub struct Context<'a> {
    pub server: &'a bridge::Server,
    pub logger: &'a bridge::Logger,
}

/// Trait plugins should implement
pub trait Plugin: Send + 'static {
    /// Called after a plugin is loaded but before it has been enabled.
    fn on_load(&mut self, _ctx: Context) {}

    /// Called when this plugin is enabled.
    fn on_enable(&mut self, _ctx: Context) {}

    /// Called when this plugin is disabled.
    fn on_disable(&mut self, _ctx: Context) {}
}

/// Metadata about your plugin
pub struct PluginMeta {
    pub name: &'static str,
    pub version: &'static str,
    pub description: Option<&'static str>,
    pub author: Option<&'static str>,
    pub website: Option<&'static str>,
    pub prefix: Option<&'static str>,
}

pub unsafe extern "C" fn trampoline_on_load<T: Plugin>(ptr: *mut (), server: *const bridge::Server) {
    unsafe {
        let plugin = &mut *(ptr as *mut T);
        let server = &*server;
        let ctx = Context {
            server,
            logger: server.get_logger(),
        };
        plugin.on_load(ctx);
    }
}

pub unsafe extern "C" fn trampoline_on_enable<T: Plugin>(ptr: *mut (), server: *const bridge::Server) {
    unsafe {
        let plugin = &mut *(ptr as *mut T);
        let server = &*server;
        let ctx = Context {
            server,
            logger: server.get_logger(),
        };
        plugin.on_enable(ctx);
    }
}

pub unsafe extern "C" fn trampoline_on_disable<T: Plugin>(ptr: *mut (), server: *const bridge::Server) {
    unsafe {
        let plugin = &mut *(ptr as *mut T);
        let server = &*server;
        let ctx = Context {
            server,
            logger: server.get_logger(),
        };
        plugin.on_disable(ctx);
    }
}


pub unsafe extern "C" fn trampoline_drop<T: Plugin>(ptr: *mut ()) {
    unsafe {
        drop(Box::from_raw(ptr as *mut T));
    }
}

/// Called by the proc macro's generated `endstone_rs_init()`.
/// Sets up the globals bridge.cpp reads.
///
/// # Safety
/// Must only be called once, from `endstone_rs_init()`, before the C++
/// shim reads any of the globals.
pub fn register_plugin<T: Plugin>(plugin: T, meta: PluginMeta) {
    let boxed = Box::new(plugin);
    let raw = Box::into_raw(boxed) as *mut ();

    unsafe {
        bridge::ENDSTONE_RS_PLUGIN_PTR = raw;

        bridge::ENDSTONE_RS_VTABLE = bridge::RsPluginVTable {
            on_load: Some(trampoline_on_load::<T>),
            on_enable: Some(trampoline_on_enable::<T>),
            on_disable: Some(trampoline_on_disable::<T>),
            drop: Some(trampoline_drop::<T>),
        };

        // We leak the CStrings intentionally — they live for the process lifetime.
        // (Could also be static strs with a wrapper, but this is simpler for now.)
        let name = CString::new(meta.name).unwrap();
        let version = CString::new(meta.version).unwrap();

        bridge::ENDSTONE_RS_META = bridge::RsPluginMeta {
            name: name.into_raw(),
            version: version.into_raw(),
            description: meta
                .description
                .map(|s| CString::new(s).unwrap().into_raw() as *const _)
                .unwrap_or(std::ptr::null()),
            author: meta
                .author
                .map(|s| CString::new(s).unwrap().into_raw() as *const _)
                .unwrap_or(std::ptr::null()),
            website: meta
                .website
                .map(|s| CString::new(s).unwrap().into_raw() as *const _)
                .unwrap_or(std::ptr::null()),
            prefix: meta
                .prefix
                .map(|s| CString::new(s).unwrap().into_raw() as *const _)
                .unwrap_or(std::ptr::null()),
        };
    }
}