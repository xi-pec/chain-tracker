use crate::define_hook;
use crate::define_hook_copy;
use crate::types::*;
use crate::vtable::VTABLE;
use crate::hooks::macros::get_hachimi_and_interceptor;

use std::ffi::c_void;
use std::collections::HashMap;

$$HOOKDEFS$$

$$COPIES$$

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct HookData {
    pub addr: *mut c_void,
    pub param_count: i32
}

pub struct HookPool {
    pub available: HashMap<i32, Vec<HookData>>,
    pub busy: HashMap<HookData, *mut Method>
}

impl HookPool {
    pub fn init() -> Self {
        let mut pool = Self {
            available: HashMap::new(),
            busy: HashMap::new()
        };

        $$HOOKINITS$$

        pool
    }

    pub fn list(&self) -> Vec<(&HookData, &*mut Method)> {
        let mut hooked = Vec::new();

        for data in &self.busy {
            hooked.push(data);
        }

        hooked
    }

    pub fn allocate(&mut self, method: *mut Method, param_count: i32) -> Option<HookData> {
        if let Some(pool) = self.available.get_mut(&param_count) {
            if let Some(hook) = pool.pop() {
                self.busy.insert(hook, method);

                Some(hook)
            } else { None }
        } else { None }
    }

    pub fn deallocate(&mut self, hook: HookData) -> bool {
        if let Some(_) = self.busy.remove(&hook) {
            self.available
                .entry(hook.param_count)
                .or_default()
                .push(hook);

            true
        } else { false }
    }
}