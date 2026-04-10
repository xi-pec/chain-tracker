mod console;
mod vtable;
mod plugin;
mod il2cpp;
mod types;
mod core;

use crate::vtable::Vtable;
use crate::plugin::PLUGIN;

#[repr(i32)]
pub enum InitResult {
    Error = 0,
    Ok = 1,
}

#[no_mangle]
pub unsafe extern "C" fn hachimi_init(vtable: *const Vtable, version: i32) -> InitResult {
    let success = plugin::init(vtable, version);
    if !success { return InitResult::Error };

    if let Some(plugin) = PLUGIN.get() {        
        plugin.console.log("Chain Tracker");
    } else { return InitResult::Error };

    InitResult::Ok
}
