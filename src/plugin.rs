use crate::types::*;
use crate::vtable::Vtable;
use crate::il2cpp::api::IAPI;
use crate::hooks::Hooks;
use crate::core::Core;

use std::sync::OnceLock;


pub struct Plugin {
    pub vtable: &'static Vtable,
    pub hachimi: Hachimi,
    pub interceptor: Interceptor,

    pub il2cpp: IAPI,
    pub core: Core,
    pub hooks: Hooks
}

impl Plugin {
    pub unsafe fn init(vtable: &'static Vtable) -> Self {
        let hachimi = (vtable.hachimi_instance)();
        let interceptor = (vtable.hachimi_get_interceptor)(hachimi);
        
        let il2cpp = IAPI::init(vtable, interceptor);
        let core = Core::init(il2cpp);
        let hooks = Hooks::init(il2cpp);

        Self { vtable, hachimi, interceptor, il2cpp, core, hooks }
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
    plugin.hooks.setup();

    true
}