use crate::types::*;
use crate::vtable::VTABLE;

pub unsafe fn get_hachimi_and_interceptor() -> (*const Hachimi, *const Interceptor) {
    let vtable = VTABLE.unwrap();
    let hachimi = (vtable.hachimi_instance)();
    let interceptor = (vtable.hachimi_get_interceptor)(hachimi);
    (hachimi, interceptor)
}

#[macro_export]
macro_rules! define_hook {
    ($name:ident, ($($arg:ident),*)) => {
        pub unsafe extern "C" fn $name(this: *mut c_void, intermediate: *mut c_void, $($arg: *mut c_void),*) -> *mut c_void {
            if let Some(vtable) = VTABLE {
                let (_, interceptor) = get_hachimi_and_interceptor();

                let trampoline = (vtable.interceptor_get_trampoline_addr)(
                    interceptor,
                    intermediate as *mut c_void
                );

                type HookSig = unsafe extern "C" fn(*mut c_void, $($arg: *mut c_void),*) -> *mut c_void;
                let original: HookSig = std::mem::transmute(trampoline);
                let result = original(this, $($arg),*);

                println!(concat!(stringify!($name), " invoked"));

                result
            } else {
                std::ptr::null_mut()
            }
        }
    };
}

#[macro_export]
macro_rules! define_hook_copy {
    ($name:ident, $target:ident, ($($arg:ident),*)) => {
        pub unsafe extern "C" fn $name(this: *mut c_void, $($arg: *mut c_void),*) -> *mut c_void {
            $target(this, $name as *mut c_void, $($arg),*)
        }
    };
}