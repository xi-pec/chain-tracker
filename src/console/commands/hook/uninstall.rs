
use crate::plugin::PLUGIN;
use crate::console::log;

use std::str::Split;

pub unsafe fn hook_uninstall(args: &mut Split<'_, &str>) {
    let image_name = match args.next() {
        Some(name) => name,
        None => return log("Missing 1st Argument (Image Name).")
    };

    let namespace = match args.next() {
        Some(name) => name,
        None => return log("Missing 2nd Argument (Namespace).")
    };

    let class_name = match args.next() {
        Some(name) => name,
        None => return log("Missing 3rd Argument (Class Name).")
    };

    let method_name = match args.next() {
        Some(name) => name,
        None => return log("Missing 4th Argument (Method Name).")
    };

    let param_count = match args.next() {
        Some(count) => match count.parse::<i32>() {
            Ok(parsed) => parsed,
            Err(_) =>  return log("Unable to parse 5th Argument (Parameter Count).") 
        }
        None => return log("Missing 5th Argument (Parameter Count).")
    };

    let plugin = PLUGIN.get().unwrap();
    let mutex = plugin.hooks.get().unwrap();
    let mut hooks = mutex.lock().unwrap();

    let Ok(image) = hooks.get_image(image_name)
    else { return log(format!("Image [{}] does not exist.", image_name)) };

    let Ok(class) = hooks.get_class(image, namespace, class_name)
    else { return log(format!("Class [{}] {}.{} does not exist.", image_name, namespace, class_name))};

    let Ok(method) = hooks.get_method(class, method_name, param_count)
    else { return log(format!("Method [{}] {}.{}::{} does not exist.", image_name, namespace, class_name, method_name))};

    let result = hooks.uninstall(method);
    
    if result { log(format!("Successfully uninstalled hook on method [{}] {}.{}::{}.", image_name, namespace, class_name, method_name))}
    else { log(format!("Could not uninstall hook on method [{}] {}.{}::{}.", image_name, namespace, class_name, method_name)) }
}