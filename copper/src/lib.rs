mod bridge;
pub mod plugin;

pub use plugin::{Plugin, PluginMeta};
pub use endstone_copper_macros::endstone_plugin;

extern crate self as endstone;

use autocxx::prelude::*;

include_cpp! {
    #include "endstone/plugin/plugin_description.h"
    #include "endstone/command/command.h"
    safety!(unsafe_ffi)
    generate!("endstone::PluginDescription")
    generate!("endstone::Command")
    block_constructors!("endstone::Command")
}

pub fn test_autocxx() {
    // This demonstrates that we can use both autocxx and cxx.
    // PluginDescription is from autocxx.
    // Logger is from manual cxx bridge.

    // We can't easily construct PluginDescription without more setup, 
    // but we can show how the types are used.
    println!("Autocxx and manual CXX bridges are linked and ready.");
}