use crate::define_hook;
use crate::define_hook_copy;
use crate::types::*;
use crate::vtable::VTABLE;
use crate::plugin::PLUGIN;
use crate::hooks::macros::get_hachimi_and_interceptor;

use std::ffi::c_void;
use std::collections::HashMap;

define_hook!(hook0, ());
define_hook!(hook1, (a1));
define_hook!(hook2, (a1, a2));
define_hook!(hook3, (a1, a2, a3));
define_hook!(hook4, (a1, a2, a3, a4));
define_hook!(hook5, (a1, a2, a3, a4, a5));
define_hook!(hook6, (a1, a2, a3, a4, a5, a6));
define_hook!(hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook!(hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook!(hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));

define_hook_copy!(hook0_0, hook0, ());
define_hook_copy!(hook0_1, hook0, ());
define_hook_copy!(hook0_2, hook0, ());
define_hook_copy!(hook0_3, hook0, ());
define_hook_copy!(hook0_4, hook0, ());
define_hook_copy!(hook0_5, hook0, ());
define_hook_copy!(hook0_6, hook0, ());
define_hook_copy!(hook0_7, hook0, ());
define_hook_copy!(hook0_8, hook0, ());
define_hook_copy!(hook0_9, hook0, ());
define_hook_copy!(hook1_0, hook1, (a1));
define_hook_copy!(hook1_1, hook1, (a1));
define_hook_copy!(hook1_2, hook1, (a1));
define_hook_copy!(hook1_3, hook1, (a1));
define_hook_copy!(hook1_4, hook1, (a1));
define_hook_copy!(hook1_5, hook1, (a1));
define_hook_copy!(hook1_6, hook1, (a1));
define_hook_copy!(hook1_7, hook1, (a1));
define_hook_copy!(hook1_8, hook1, (a1));
define_hook_copy!(hook1_9, hook1, (a1));
define_hook_copy!(hook2_0, hook2, (a1, a2));
define_hook_copy!(hook2_1, hook2, (a1, a2));
define_hook_copy!(hook2_2, hook2, (a1, a2));
define_hook_copy!(hook2_3, hook2, (a1, a2));
define_hook_copy!(hook2_4, hook2, (a1, a2));
define_hook_copy!(hook2_5, hook2, (a1, a2));
define_hook_copy!(hook2_6, hook2, (a1, a2));
define_hook_copy!(hook2_7, hook2, (a1, a2));
define_hook_copy!(hook2_8, hook2, (a1, a2));
define_hook_copy!(hook2_9, hook2, (a1, a2));
define_hook_copy!(hook3_0, hook3, (a1, a2, a3));
define_hook_copy!(hook3_1, hook3, (a1, a2, a3));
define_hook_copy!(hook3_2, hook3, (a1, a2, a3));
define_hook_copy!(hook3_3, hook3, (a1, a2, a3));
define_hook_copy!(hook3_4, hook3, (a1, a2, a3));
define_hook_copy!(hook3_5, hook3, (a1, a2, a3));
define_hook_copy!(hook3_6, hook3, (a1, a2, a3));
define_hook_copy!(hook3_7, hook3, (a1, a2, a3));
define_hook_copy!(hook3_8, hook3, (a1, a2, a3));
define_hook_copy!(hook3_9, hook3, (a1, a2, a3));
define_hook_copy!(hook4_0, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook4_1, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook4_2, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook4_3, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook4_4, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook4_5, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook4_6, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook4_7, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook4_8, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook4_9, hook4, (a1, a2, a3, a4));
define_hook_copy!(hook5_0, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook5_1, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook5_2, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook5_3, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook5_4, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook5_5, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook5_6, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook5_7, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook5_8, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook5_9, hook5, (a1, a2, a3, a4, a5));
define_hook_copy!(hook6_0, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook6_1, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook6_2, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook6_3, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook6_4, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook6_5, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook6_6, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook6_7, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook6_8, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook6_9, hook6, (a1, a2, a3, a4, a5, a6));
define_hook_copy!(hook7_0, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook7_1, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook7_2, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook7_3, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook7_4, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook7_5, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook7_6, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook7_7, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook7_8, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook7_9, hook7, (a1, a2, a3, a4, a5, a6, a7));
define_hook_copy!(hook8_0, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook8_1, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook8_2, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook8_3, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook8_4, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook8_5, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook8_6, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook8_7, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook8_8, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook8_9, hook8, (a1, a2, a3, a4, a5, a6, a7, a8));
define_hook_copy!(hook9_0, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));
define_hook_copy!(hook9_1, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));
define_hook_copy!(hook9_2, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));
define_hook_copy!(hook9_3, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));
define_hook_copy!(hook9_4, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));
define_hook_copy!(hook9_5, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));
define_hook_copy!(hook9_6, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));
define_hook_copy!(hook9_7, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));
define_hook_copy!(hook9_8, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));
define_hook_copy!(hook9_9, hook9, (a1, a2, a3, a4, a5, a6, a7, a8, a9));

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct HookData {
    pub addr: *mut c_void,
    pub param_count: i32
}

pub struct HookPool {
    pub available: HashMap<i32, Vec<HookData>>,
    pub busy: HashMap<HookData, *mut MethodInfo>
}

impl HookPool {
    pub fn init() -> Self {
        let mut pool = Self {
            available: HashMap::new(),
            busy: HashMap::new()
        };

        pool.available.insert(0, vec![
            HookData { addr: hook0_0 as *mut c_void, param_count: 0 },
            HookData { addr: hook0_1 as *mut c_void, param_count: 0 },
            HookData { addr: hook0_2 as *mut c_void, param_count: 0 },
            HookData { addr: hook0_3 as *mut c_void, param_count: 0 },
            HookData { addr: hook0_4 as *mut c_void, param_count: 0 },
            HookData { addr: hook0_5 as *mut c_void, param_count: 0 },
            HookData { addr: hook0_6 as *mut c_void, param_count: 0 },
            HookData { addr: hook0_7 as *mut c_void, param_count: 0 },
            HookData { addr: hook0_8 as *mut c_void, param_count: 0 },
            HookData { addr: hook0_9 as *mut c_void, param_count: 0 }
        ]);

        pool.available.insert(1, vec![
            HookData { addr: hook1_0 as *mut c_void, param_count: 1 },
            HookData { addr: hook1_1 as *mut c_void, param_count: 1 },
            HookData { addr: hook1_2 as *mut c_void, param_count: 1 },
            HookData { addr: hook1_3 as *mut c_void, param_count: 1 },
            HookData { addr: hook1_4 as *mut c_void, param_count: 1 },
            HookData { addr: hook1_5 as *mut c_void, param_count: 1 },
            HookData { addr: hook1_6 as *mut c_void, param_count: 1 },
            HookData { addr: hook1_7 as *mut c_void, param_count: 1 },
            HookData { addr: hook1_8 as *mut c_void, param_count: 1 },
            HookData { addr: hook1_9 as *mut c_void, param_count: 1 }
        ]);

        pool.available.insert(2, vec![
            HookData { addr: hook2_0 as *mut c_void, param_count: 2 },
            HookData { addr: hook2_1 as *mut c_void, param_count: 2 },
            HookData { addr: hook2_2 as *mut c_void, param_count: 2 },
            HookData { addr: hook2_3 as *mut c_void, param_count: 2 },
            HookData { addr: hook2_4 as *mut c_void, param_count: 2 },
            HookData { addr: hook2_5 as *mut c_void, param_count: 2 },
            HookData { addr: hook2_6 as *mut c_void, param_count: 2 },
            HookData { addr: hook2_7 as *mut c_void, param_count: 2 },
            HookData { addr: hook2_8 as *mut c_void, param_count: 2 },
            HookData { addr: hook2_9 as *mut c_void, param_count: 2 }
        ]);

        pool.available.insert(3, vec![
            HookData { addr: hook3_0 as *mut c_void, param_count: 3 },
            HookData { addr: hook3_1 as *mut c_void, param_count: 3 },
            HookData { addr: hook3_2 as *mut c_void, param_count: 3 },
            HookData { addr: hook3_3 as *mut c_void, param_count: 3 },
            HookData { addr: hook3_4 as *mut c_void, param_count: 3 },
            HookData { addr: hook3_5 as *mut c_void, param_count: 3 },
            HookData { addr: hook3_6 as *mut c_void, param_count: 3 },
            HookData { addr: hook3_7 as *mut c_void, param_count: 3 },
            HookData { addr: hook3_8 as *mut c_void, param_count: 3 },
            HookData { addr: hook3_9 as *mut c_void, param_count: 3 }
        ]);

        pool.available.insert(4, vec![
            HookData { addr: hook4_0 as *mut c_void, param_count: 4 },
            HookData { addr: hook4_1 as *mut c_void, param_count: 4 },
            HookData { addr: hook4_2 as *mut c_void, param_count: 4 },
            HookData { addr: hook4_3 as *mut c_void, param_count: 4 },
            HookData { addr: hook4_4 as *mut c_void, param_count: 4 },
            HookData { addr: hook4_5 as *mut c_void, param_count: 4 },
            HookData { addr: hook4_6 as *mut c_void, param_count: 4 },
            HookData { addr: hook4_7 as *mut c_void, param_count: 4 },
            HookData { addr: hook4_8 as *mut c_void, param_count: 4 },
            HookData { addr: hook4_9 as *mut c_void, param_count: 4 }
        ]);

        pool.available.insert(5, vec![
            HookData { addr: hook5_0 as *mut c_void, param_count: 5 },
            HookData { addr: hook5_1 as *mut c_void, param_count: 5 },
            HookData { addr: hook5_2 as *mut c_void, param_count: 5 },
            HookData { addr: hook5_3 as *mut c_void, param_count: 5 },
            HookData { addr: hook5_4 as *mut c_void, param_count: 5 },
            HookData { addr: hook5_5 as *mut c_void, param_count: 5 },
            HookData { addr: hook5_6 as *mut c_void, param_count: 5 },
            HookData { addr: hook5_7 as *mut c_void, param_count: 5 },
            HookData { addr: hook5_8 as *mut c_void, param_count: 5 },
            HookData { addr: hook5_9 as *mut c_void, param_count: 5 }
        ]);

        pool.available.insert(6, vec![
            HookData { addr: hook6_0 as *mut c_void, param_count: 6 },
            HookData { addr: hook6_1 as *mut c_void, param_count: 6 },
            HookData { addr: hook6_2 as *mut c_void, param_count: 6 },
            HookData { addr: hook6_3 as *mut c_void, param_count: 6 },
            HookData { addr: hook6_4 as *mut c_void, param_count: 6 },
            HookData { addr: hook6_5 as *mut c_void, param_count: 6 },
            HookData { addr: hook6_6 as *mut c_void, param_count: 6 },
            HookData { addr: hook6_7 as *mut c_void, param_count: 6 },
            HookData { addr: hook6_8 as *mut c_void, param_count: 6 },
            HookData { addr: hook6_9 as *mut c_void, param_count: 6 }
        ]);

        pool.available.insert(7, vec![
            HookData { addr: hook7_0 as *mut c_void, param_count: 7 },
            HookData { addr: hook7_1 as *mut c_void, param_count: 7 },
            HookData { addr: hook7_2 as *mut c_void, param_count: 7 },
            HookData { addr: hook7_3 as *mut c_void, param_count: 7 },
            HookData { addr: hook7_4 as *mut c_void, param_count: 7 },
            HookData { addr: hook7_5 as *mut c_void, param_count: 7 },
            HookData { addr: hook7_6 as *mut c_void, param_count: 7 },
            HookData { addr: hook7_7 as *mut c_void, param_count: 7 },
            HookData { addr: hook7_8 as *mut c_void, param_count: 7 },
            HookData { addr: hook7_9 as *mut c_void, param_count: 7 }
        ]);

        pool.available.insert(8, vec![
            HookData { addr: hook8_0 as *mut c_void, param_count: 8 },
            HookData { addr: hook8_1 as *mut c_void, param_count: 8 },
            HookData { addr: hook8_2 as *mut c_void, param_count: 8 },
            HookData { addr: hook8_3 as *mut c_void, param_count: 8 },
            HookData { addr: hook8_4 as *mut c_void, param_count: 8 },
            HookData { addr: hook8_5 as *mut c_void, param_count: 8 },
            HookData { addr: hook8_6 as *mut c_void, param_count: 8 },
            HookData { addr: hook8_7 as *mut c_void, param_count: 8 },
            HookData { addr: hook8_8 as *mut c_void, param_count: 8 },
            HookData { addr: hook8_9 as *mut c_void, param_count: 8 }
        ]);

        pool.available.insert(9, vec![
            HookData { addr: hook9_0 as *mut c_void, param_count: 9 },
            HookData { addr: hook9_1 as *mut c_void, param_count: 9 },
            HookData { addr: hook9_2 as *mut c_void, param_count: 9 },
            HookData { addr: hook9_3 as *mut c_void, param_count: 9 },
            HookData { addr: hook9_4 as *mut c_void, param_count: 9 },
            HookData { addr: hook9_5 as *mut c_void, param_count: 9 },
            HookData { addr: hook9_6 as *mut c_void, param_count: 9 },
            HookData { addr: hook9_7 as *mut c_void, param_count: 9 },
            HookData { addr: hook9_8 as *mut c_void, param_count: 9 },
            HookData { addr: hook9_9 as *mut c_void, param_count: 9 }
        ]);

        pool
    }

    pub fn list(&self) -> Vec<(&HookData, &*mut MethodInfo)> {
        let mut hooked = Vec::new();

        for data in &self.busy {
            hooked.push(data);
        }

        hooked
    }

    pub fn allocate(&mut self, method: *mut MethodInfo, param_count: i32) -> Option<HookData> {
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