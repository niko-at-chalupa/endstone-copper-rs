mod ffi;
pub mod plugin;

pub use plugin::{Plugin, PluginMeta};
pub use endstone_copper_macros::endstone_plugin;

extern crate self as endstone;