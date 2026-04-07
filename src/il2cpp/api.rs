use crate::vtable::Vtable;
use crate::types::*;
use crate::il2cpp::types::*;

use std::ffi::{CString, c_void};

#[derive(Copy, Clone)]
pub struct IAPI {
    pub vtable: &'static Vtable,
    pub interceptor: Interceptor,
}

impl IAPI {
    pub fn init(vtable: &'static Vtable, interceptor: Interceptor) -> Self {
        Self { vtable, interceptor }
    }

    pub unsafe fn interceptor_hook(&self, original: Address, hook: Address) -> Result<Address, i32> {
        let trampoline = (self.vtable.interceptor_hook)(self.interceptor, original, hook);

        if trampoline != 0 { Ok(trampoline) }
        else { Err(1) } // Trampoline not found (original fn could not be hooked?)
    }

    pub unsafe fn interceptor_unhook(&self, hook: Address) -> Result<Address, i32> {
        let original = (self.vtable.interceptor_unhook)(self.interceptor, hook);

        if original != 0 { Ok(original) }
        else { Err(1) } // Original not found (hook has no corresponding original fn)
    }

    pub unsafe fn get_trampoline(&self, hook: Address) -> Result<Address, i32> {
        let trampoline = (self.vtable.interceptor_get_trampoline_addr)(self.interceptor, hook);

        if trampoline != 0 { Ok(trampoline) }
        else { Err(1) } // Trampoline not found
    }

    pub unsafe fn get_image(&self, image_name: &str) -> Result<IImage, i32> {
        let image_name = CString::new(image_name).unwrap();

        let image = (self.vtable.il2cpp_get_assembly_image)(
            image_name.as_ptr() as String
        ) as IImage;

        if !image.is_null() { Ok(image) }
        else { Err(1) } // Null Image 
    }

    pub unsafe fn get_class(&self, image: IImage, namespace: &str, class_name: &str) -> Result<IClass, i32> {
        let namespace = CString::new(namespace).unwrap();
        let class_name = CString::new(class_name).unwrap();

        let class = (self.vtable.il2cpp_get_class)(
            image,
            namespace.as_ptr() as String,
            class_name.as_ptr() as String
        ) as IClass;

        if !class.is_null() { Ok(class) }
        else { Err(1) } // Null Class 
    }

    pub unsafe fn get_object_class(&self, object: IObject) -> Result<IClass, i32> {
        // polyfill implementation
        if object.is_null() { crate::console::log("null obj"); return Err(1) }

        let ptr = object as *mut IObjectP;
        if ptr.is_null() { return Err(1) }

        let object = &*ptr;
        let class = object.data.class;

        if !class.is_null() { Ok(class) }
        else { crate::console::log("null obj class"); Err(2) }
    }

    pub unsafe fn get_method(&self, class: IClass, method_name: &str, args: i32) -> Result<IMethod, i32> {
        let method_name = CString::new(method_name).unwrap();
        let method = (self.vtable.il2cpp_get_method_cached)(
            class,
            method_name.as_ptr() as String,
            args
        ) as IMethod;

        if !method.is_null() { Ok(method) }
        else { Err(1) } // Null Method 
    }

    pub unsafe fn get_method_addr(&self, class: IClass, method_name: &str, args: i32) -> Result<Address, i32> {
        let method_name = CString::new(method_name).unwrap();
        let address = (self.vtable.il2cpp_get_method_addr_cached)(
            class,
            method_name.as_ptr() as String,
            args
        ) as Address;

        Ok(address)
    }

    pub unsafe fn get_field(&self, class: IClass, field_name: &str) -> Result<IField, i32> {
        let field_name = CString::new(field_name).unwrap();
        let field = (self.vtable.il2cpp_get_field_from_name)(
            class,
            field_name.as_ptr() as String,
        ) as IField;

        if !field.is_null() { Ok(field) }
        else { Err(1) }
    }

    pub unsafe fn get_field_value(&self, object: IObject, field: IField) -> Result<*mut c_void, i32> {
        let mut value: *mut c_void = std::ptr::null_mut();
        (self.vtable.il2cpp_get_field_value)(object, field, &mut value as *mut _ as *mut c_void);

        Ok(value)
    }

    pub unsafe fn get_array_length(&self, arr: IArray) -> Result<u32, i32> {
        // for modified chimi
        // let size = (self.vtable.il2cpp_get_array_length)(arr);
        // Ok(size)
        
        // polyfill implementation
        let ptr = arr as *mut IArrayP;
        if ptr.is_null() { return Err(1) }

        let arr = &*ptr;
        let length = arr.max_length as u32;

        Ok(length)
    }

    pub unsafe fn get_array_element_size(&self, class: IClass) -> Result<i32, i32> {
        // for modified chimi
        // let size = (self.vtable.il2cpp_get_array_element_size)(class);
        // Ok(size)

        // polyfill implementation
        let ptr = class as *mut IClassP;
        if ptr.is_null() { return Err(1) }

        let class = &*ptr;
        let size = class.element_size as i32;

        Ok(size)
    }

    pub unsafe fn unbox(&self, object: IObject) -> Result<*mut c_void, i32> {
        let data = (self.vtable.il2cpp_unbox)(object);
        Ok(data)
    }
}