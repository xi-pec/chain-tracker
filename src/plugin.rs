use crate::vtable::Vtable;
use crate::il2cpp::api::IAPI;
use crate::core::Core;
use crate::console::Console;

use std::sync::OnceLock;


pub struct Plugin {
    pub il2cpp: IAPI,
    pub core: Core,
    pub console: Console,
}

impl Plugin {
    pub unsafe fn init(vtable: &'static Vtable) -> Self {
        let hachimi = (vtable.hachimi_instance)();
        let interceptor = (vtable.hachimi_get_interceptor)(hachimi);
        
        let il2cpp = IAPI::init(vtable, interceptor);
        let core = Core::init(il2cpp);
        let console = Console::init();

        Self { il2cpp, core, console }
    }
}

unsafe impl Send for Plugin {}
unsafe impl Sync for Plugin {}

pub static PLUGIN: OnceLock<Plugin> = OnceLock::new();

pub unsafe fn init(vtable: *const Vtable, version: i32) -> bool {
    if vtable.is_null() || version < 2 { return false }
    let vtable = Vtable::init(vtable);
    
    let Ok(_) = PLUGIN.set(Plugin::init(vtable))
    else { return false };

    let plugin = PLUGIN.get().unwrap();
    plugin.core.setup();

    true
}