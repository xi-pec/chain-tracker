use crate::plugin::PLUGIN;
use crate::console::log;

use std::ffi::{CStr};

pub unsafe fn list() {
    let plugin = PLUGIN.get().unwrap();
    let mutex = plugin.hooks.get().unwrap();
    let hooks = mutex.lock().unwrap();

    let w_addr = 15;
    let w_image = 30;
    let w_namespace = 30;
    let w_class = 30;
    let w_method = 30;
    let w_params = 5;

    log(format!(
        "{:<w1$} {:<w2$} {:<w3$} {:<w4$} {:<w5$} {:<w6$}",
        "HOOK ADDR", "IMAGE", "NAMESPACE", "CLASS", "METHOD", "PARAMS",
        w1=w_addr, w2=w_image, w3=w_namespace, w4=w_class, w5=w_method, w6=w_params
    ));

    let fmt = |s: String, width: usize| {
        if s.len() > width {
            format!("{}..", &s[..width - 2])
        } else {
            format!("{:<width$}", s, width = width)
        }
    };

    for (hook, method) in hooks.list() {
        let addr = hook.addr as usize;

        let param_count = hooks.get_method_param_count(*method);

        let method_name = CStr::from_ptr(hooks.get_method_name(*method).unwrap()).to_string_lossy().to_owned();

        let class = hooks.get_class_from_method(*method).unwrap();
        let class_name = CStr::from_ptr(hooks.get_class_name(class).unwrap()).to_string_lossy().to_owned();

        let namespace = CStr::from_ptr(hooks.get_namespace_from_class(class).unwrap()).to_string_lossy().to_owned();

        let image = hooks.get_image_from_class(class).unwrap();
        let image_name = CStr::from_ptr(hooks.get_image_name(image).unwrap()).to_string_lossy().to_owned();

        log(format!(
            "{:<w1$} {} {} {} {} {:<w6$}",
            format!("0x{:x}", addr as usize),
            fmt(image_name.to_string(), w_image),
            fmt(namespace.to_string(), w_namespace),
            fmt(class_name.to_string(), w_class),
            fmt(method_name.to_string(), w_method),
            fmt(param_count.to_string(), w_params),
            w1=w_addr, w6=w_params
        ));
    }
}