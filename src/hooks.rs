pub mod http;

use crate::types::*;
use crate::il2cpp::api::IAPI;

pub struct Hooks {
    pub il2cpp: IAPI
}

impl Hooks {
    pub fn init(il2cpp: IAPI) -> Self {
        Self { il2cpp }
    }

    pub unsafe fn setup(&self) {
        http::init();
    }
    
    pub unsafe fn install(&self, image: &str, namespace: &str, class: &str, method: &str, param_count: i32, hook: Address) -> bool {
        let Ok(image) = self.il2cpp.get_image(image)
        else { return false };

        let Ok(class) = self.il2cpp.get_class(image, namespace, class)
        else { return false };

        let Ok(method) = self.il2cpp.get_method_addr(class, method, param_count)
        else { return false };
        
        let Ok(trampoline) = self.il2cpp.interceptor_hook(method, hook)
        else { return false };

        trampoline != 0
    }

    pub unsafe fn uninstall(&self, hook: Address) -> bool {
        let Ok(original) = self.il2cpp.interceptor_unhook(hook)
        else { return false };

        original != 0
    }
}