mod console;
mod vtable;
mod hooks;
mod types;
mod plugin;

use std::ffi::CStr;

use crate::vtable::Vtable;
use crate::plugin::PLUGIN;
use crate::console::log;

#[repr(i32)]
pub enum InitResult {
    Error = 0,
    Ok = 1,
}

#[no_mangle]
pub unsafe extern "C" fn hachimi_init(vtable: *const Vtable, version: i32) -> InitResult {
    console::init_console();

    let success = plugin::init(vtable, version);
    if !success { return InitResult::Error };

    if let Some(plugin) = PLUGIN.get() {
        log("UmaSpy v0.1.0");

        if let Some(mutex) = plugin.hooks.get() {
            let mut hooks = mutex.lock().unwrap();

            let image = hooks.get_image("umamusume");
            let class = hooks.get_class(image.unwrap(), "Gallop", "DialogCircleItemDonate");

            let method1 = hooks.get_method(class.unwrap(), "OnClickPlusButton", 0);
            let method2 = hooks.get_method(class.unwrap(), "OnClickMinusButton", 0);

            hooks.install(method1.unwrap(), 0);
            hooks.install(method2.unwrap(), 0);

            
            // hooks.uninstall(method2.unwrap());
            // hooks.install(method2.unwrap(), 0);
        }
    } else { return InitResult::Error }

    InitResult::Ok
}
