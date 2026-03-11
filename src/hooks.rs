pub mod macros;
pub mod generated;

use crate::hooks::generated::HookData;
use crate::types::*;
use crate::vtable::Vtable;
use crate::console::log;

use generated::HookPool;

use std::ffi::{CString, CStr, c_char};
use std::collections::HashMap;

pub struct Hooks {
    pub vtable: &'static Vtable,
    pub interceptor: *const Interceptor,
    pub pool: HookPool,

    installed: HashMap<*mut MethodInfo, HookData>,
    references: HashMap<usize, *mut MethodInfo>
}

impl Hooks {
    pub fn init(vtable: &'static Vtable, interceptor: *const Interceptor) -> Self {
        Self { 
            vtable, interceptor,
            pool: HookPool::init(),

            installed: HashMap::new(),
            references: HashMap::new()
        }
    }

    pub unsafe fn get_image(&self, image_name: &str) -> Result<*const Image, i32> {
        let image_name = CString::new(image_name).unwrap();

        let image = (self.vtable.il2cpp_get_assembly_image)(
            image_name.as_ptr()
        ) as *const Image;

        if !image.is_null() { Ok(image) }
        else { Err(1) } // Null Image 
    }

    pub unsafe fn get_image_from_class(&self, class: *mut Class) -> Result<*const Image, i32> {
        let image = (self.vtable.il2cpp_get_image_from_class)(class);

        if !image.is_null() { Ok(image) }
        else { Err(1) } // Null Image 
    }

    pub unsafe fn get_image_name(&self, image: *const Image) -> Result<*const c_char, i32> {
        let image_name = (self.vtable.il2cpp_get_image_name)(image);

        if !image_name.is_null() { Ok(image_name) }
        else { Err(1) } // Null Image Name
    }

    pub unsafe fn get_namespace_from_class(&self, class: *mut Class) -> Result<*const c_char, i32> {
        let namespace = (self.vtable.il2cpp_get_namespace_from_class)(class);

        if !namespace.is_null() { Ok(namespace) }
        else { Err(1) } // Null Namespace
    }

    pub unsafe fn get_classes(&self, image: *const Image) -> Result<Vec<*mut Class>, i32> {
        let classes = (self.vtable.il2cpp_get_classes)(image).into_vec();

        for class in &classes {
            if class.is_null() { return Err(1) } // Null Class 
        }

        Ok(classes)
    }

    pub unsafe fn get_class(&self, image: *const Image, namespace: &str, class_name: &str) -> Result<*mut Class, i32> {
        let namespace = CString::new(namespace).unwrap();
        let class_name = CString::new(class_name).unwrap();

        let class = (self.vtable.il2cpp_get_class)(
            image,
            namespace.as_ptr(),
            class_name.as_ptr()
        ) as *mut Class;

        if !class.is_null() { Ok(class) }
        else { Err(1) } // Null Class 
    }

    pub unsafe fn get_class_from_method(&self, method: *mut MethodInfo) -> Result<*mut Class, i32> {
        let class = (self.vtable.il2cpp_get_class_from_method)(method);

        if !class.is_null() { Ok(class) }
        else { Err(1) } // Null Class
    }

    pub unsafe fn get_class_name(&self, class: *mut Class) -> Result<*const c_char, i32> {
        let class_name = (self.vtable.il2cpp_get_class_name)(class);

        if !class_name.is_null() { Ok(class_name) }
        else { Err(1) } // Null Class Name
    }

    pub unsafe fn get_methods(&self, class: *mut Class) -> Result<Vec<*mut MethodInfo>, i32> {
        let methods = (self.vtable.il2cpp_get_methods)(class).into_vec();
        for method in &methods {
            if method.is_null() { return Err(1) } // Null Class 
        };

        Ok(methods)
    }

    pub unsafe fn get_method(&self, class: *mut Class, method_name: &str, args: i32) -> Result<*mut MethodInfo, i32> {
        let method_name = CString::new(method_name).unwrap();
        let method = (self.vtable.il2cpp_get_method_cached)(
            class,
            method_name.as_ptr(),
            args
        ) as *mut MethodInfo;

        if !method.is_null() { Ok(method) }
        else { Err(1) } // Null Method 
    }
    
    pub unsafe fn get_method_name(&self, method: *mut MethodInfo) -> Result<*const c_char, i32> {
        let class_name = (self.vtable.il2cpp_get_method_name)(method);

        if !class_name.is_null() { Ok(class_name) }
        else { Err(1) } // Null Method Name
    }

    pub unsafe fn get_method_addr(&self, method: *mut MethodInfo) -> Result<*mut MethodAddress, i32> {
        let class = self.get_class_from_method(method);
        if class.is_err() { return Err(1) } // Null Class

        let name = self.get_method_name(method);
        if name.is_err() { return Err(2) } // Null Method Name

        let param_count = self.get_method_param_count(method);

        let addr = (self.vtable.il2cpp_get_method_addr)(class.unwrap(), name.unwrap(), param_count);

        if !addr.is_null() { Ok(addr) }
        else { Err(3) } // Null Method Address
    }

    pub unsafe fn get_method_param_count(&self, method: *mut MethodInfo) -> i32 {
        let args = (self.vtable.il2cpp_get_method_param_count)(method);
        
        args as i32
    }

    pub unsafe fn list(&self) -> Vec<(&HookData, &*mut MethodInfo)> {
        self.pool.list()
    }

    pub unsafe fn install(&mut self, method: *mut MethodInfo, param_count: i32) -> bool {
        if self.installed.contains_key(&method) { return false }

        if let Some(hook) = self.pool.allocate(method, param_count) {
            let Ok(addr) = self.get_method_addr(method)
            else { return false };

            let trampoline = (self.vtable.interceptor_hook)(self.interceptor, addr, hook.addr);

            if trampoline != 0 {
                self.installed.insert(method, hook);
                self.references.insert(trampoline, method);

                true
            } else {
                self.pool.deallocate(hook);

                false
            }
        } else { false }
    }

    pub unsafe fn uninstall(&mut self, method: *mut MethodInfo) -> bool {
        let hook = match self.installed.get(&method) {
            Some(h ) => h,
            None => return false
        };

        let trampoline = (self.vtable.interceptor_get_trampoline_addr)(self.interceptor, hook.addr);
        if trampoline == 0 { return false }

        if self.pool.deallocate(*hook) {
            (self.vtable.interceptor_unhook)(self.interceptor, hook.addr);
            self.installed.remove_entry(&method);
            self.references.remove_entry(&trampoline);

            true
        } else { false }
    }

    pub unsafe fn callback(&mut self, trampoline: usize) {
        let to_str = |ptr| CStr::from_ptr(ptr).to_string_lossy().to_owned();

        let method = match self.references.get(&trampoline) {
            Some(m) => *m,
            None => return log(format!("Could not find reference for address {}", trampoline)) 
        };

        let method_name = to_str(self.get_method_name(method).unwrap());

        let class = self.get_class_from_method(method).unwrap();
        let class_name = to_str(self.get_class_name(class).unwrap());

        let namespace = to_str(self.get_namespace_from_class(class).unwrap());

        let image = self.get_image_from_class(class).unwrap();
        let image_name = to_str(self.get_image_name(image).unwrap());

        log(format!("[{}] {}.{}::{} invoked.", image_name, namespace, class_name, method_name));
    }
}