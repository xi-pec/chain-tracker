use crate::il2cpp::types::{IArray, IArrayP, IObject};
use crate::il2cpp::api::IAPI;

use std::ffi::c_void;

pub struct Utils {
    il2cpp: IAPI
}

impl Utils {
    pub fn init(il2cpp: IAPI) -> Self {
        Self { il2cpp }
    }

    pub unsafe fn get_field_data(&self, object: IObject, field_name: &str) -> Result<*mut c_void, i32> {
        if object.is_null() { crate::console::log("w"); return Err(1) }
        
        let Ok(object_class) = self.il2cpp.get_object_class(object)
        else { crate::console::log("x"); return Err(2) };

        let Ok(field) = self.il2cpp.get_field(object_class, field_name)
        else { crate::console::log("y"); return Err(3) };

        let Ok(value) = self.il2cpp.get_field_value(object, field)     
        else { crate::console::log("z"); return Err(4) };

        Ok(value)   
    }

    pub unsafe fn get_vector(&self, array: IArray) -> Result<Vec<*mut c_void>, i32> {
        if array.is_null() { crate::console::log("a"); return Err(1) }
        
        let Ok(length) = self.il2cpp.get_array_length(array)
        else { crate::console::log("a"); return Err(2) };

        let Ok(array_class) = self.il2cpp.get_object_class(array)
        else { crate::console::log("a"); return Err(3) };

        let Ok(size) = self.il2cpp.get_array_element_size(array_class)
        else { crate::console::log("a"); return Err(4) };

        let start = (array as *mut IArrayP).add(1) as *mut c_void;
        
        let mut vec = Vec::new();
        for i in 0..length {
            let ptr = start.add(i as usize * size as usize);
            let item = *(ptr as *mut *mut c_void);
            vec.push(item);
        }

        Ok(vec)
    }
}