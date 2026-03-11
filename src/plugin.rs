use crate::{
    hooks::Hooks, types::{Hachimi, Interceptor}, vtable::Vtable
};

use std::sync::{OnceLock, Mutex};

pub struct Plugin {
    pub vtable: &'static Vtable,
    pub hachimi: *const Hachimi,
    pub interceptor: *const Interceptor,

    pub hooks: OnceLock<Mutex<Hooks>>
}

impl Plugin {
    pub unsafe fn init(vtable: &'static Vtable) -> Self {
        let hachimi = (vtable.hachimi_instance)();
        let interceptor = (vtable.hachimi_get_interceptor)(hachimi);
        
        let hooks = OnceLock::new();
        let _ = hooks.set(Mutex::new(Hooks::init(vtable, interceptor)));

        Self { vtable, hachimi, interceptor, hooks }
    }
}

unsafe impl Send for Plugin {}
unsafe impl Sync for Plugin {}

pub static PLUGIN: OnceLock<Plugin> = OnceLock::new();

pub unsafe fn init(vtable: *const Vtable, version: i32) -> bool {
    if vtable.is_null() || version < 2 { return false }
    let vtable = Vtable::init(vtable);
    
    let _ = PLUGIN.set(Plugin::init(vtable));

    true
}