use crate::plugin::PLUGIN;
use crate::types::*;

use std::ffi::c_void;

type Hook = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void) -> *mut c_void;
unsafe extern "C" fn hook(this: *mut c_void, bytes: *mut c_void, offset: *mut c_void, resolver: *mut c_void, size: *mut c_void) -> *mut c_void {
    let Some(plugin) = PLUGIN.get()
    else { return std::ptr::null_mut() };

    let Ok(trampoline) = plugin.il2cpp.get_trampoline(hook as Address)
    else { return std::ptr::null_mut() };

    let original: Hook = std::mem::transmute(trampoline);
    let result = original(this, bytes, offset, resolver, size);

    let Some(chain_events) = plugin.core.mdb.chain_events.get()
    else { return result };

    // response > data > unchecked_event_array
    let Ok(unchecked_event_array_vector) = plugin.core.utils.get_field_data(result, "data")
        .and_then(|data| plugin.core.utils.get_field_data(data, "unchecked_event_array"))
        .and_then(|unchecked_event_array| plugin.core.utils.get_vector(unchecked_event_array))
    else { return result };
    
    let console = &plugin.console;
    let ui = &console.ui;

    for event_info in unchecked_event_array_vector {
        // unchecked_event_array[i] > story_id
        let Some(chain_event_data) = plugin.core.utils
            .get_field_data(event_info, "story_id")
            .ok()
            .and_then(|id| chain_events.get(&(id as i64)))
        else { continue };

        
        // unchecked_event_array[i] > event_contents_info > support_card_id
        let Ok(id) = plugin.core.utils.get_field_data(event_info, "event_contents_info")
            .and_then(|event_contents_info| plugin.core.utils.get_field_data(event_contents_info, "support_card_id"))
        else { continue };

        let id = id as i64;
        let current = chain_event_data.current_progress;
        let max = chain_event_data.max_progress;

        ui.update(id, current, max);
        console.update();
    }

    result
}

pub unsafe fn init(image: &str, namespace: &str, class: &str) {
    let Some(plugin) = PLUGIN.get()
    else { return };

    plugin.core.hooks.install(
        image, namespace, class,
        "Deserialize", 4,
        hook as Address
    );
}