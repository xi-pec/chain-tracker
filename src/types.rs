use crate::vtable::VTABLE;

#[repr(C)] pub struct Hachimi { _unused: [u8; 0] }
#[repr(C)] pub struct Interceptor { _unused: [u8; 0] }
#[repr(C)] pub struct Image { _unused: [u8; 0] }
#[repr(C)] pub struct Class { _unused: [u8; 0] }
#[repr(C)] pub struct MethodInfo { _unused: [u8; 0] }
#[repr(C)] pub struct MethodAddress { _unused: [u8; 0] }
#[repr(C)] pub struct TypeEnum { _unused: [u8; 0] }
#[repr(C)] pub struct Field { _unused: [u8; 0] }
#[repr(C)] pub struct Object { _unused: [u8; 0] }
#[repr(C)] pub struct Thread { _unused: [u8; 0] }
#[repr(C)] pub struct Array { _unused: [u8; 0] }

#[repr(C)]
pub struct List<Type> {
    pub data: *mut Type,
    pub size: usize,
}

impl<Type> List<Type> {
    pub fn as_slice(&self) -> &[Type] {
        if self.data.is_null() || self.size == 0 {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(self.data, self.size) }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<Type> std::ops::Deref for List<Type> {
    type Target = [Type];
    
    fn deref(&self) -> &[Type] {
        self.as_slice()
    }
}

impl<Type: Clone> List<Type> {
    pub fn into_vec(self) -> Vec<Type> {
        let v = self.as_slice().to_vec();
        v
    }
}

impl<Type> Drop for List<Type> {
    fn drop(&mut self) {
        let ptr = self.data;
        if !self.data.is_null() {
            self.data = std::ptr::null_mut();

            unsafe {
                if let Some(vtable) = VTABLE {
                    (vtable.list_free)(ptr as *mut usize, self.size);
                }
            }
        }
    }
}