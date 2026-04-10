use crate::plugin::PLUGIN;
use crate::types::*;
use crate::console::ui::{Rarity, Type};

use std::collections::HashMap;
use std::ffi::c_void;

type SingleModeBreedersLoadHook = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void) -> *mut c_void;

#[inline(never)]
#[no_mangle]
unsafe extern "C" fn single_mode_breeders_load_hook(this: *mut c_void, bytes: *mut c_void, offset: *mut c_void, resolver: *mut c_void, size: *mut c_void) -> *mut c_void {
    let Some(plugin) = &mut PLUGIN.get()
    else { return std::ptr::null_mut() };

    let Ok(trampoline) = plugin.il2cpp.get_trampoline(single_mode_breeders_load_hook as Address)
    else { return std::ptr::null_mut() };

    let original: SingleModeBreedersLoadHook = std::mem::transmute(trampoline);
    let result = original(this, bytes, offset, resolver, size);

    let Some(chara_names) = plugin.core.mdb.chara_names.get()
    else { return result };

    let Some(support_cards) = plugin.core.mdb.support_cards.get()
    else { return result };

    // response > data > single_mode_load_common > chara_info > support_card_array
    let Ok(support_card_array_vector) = plugin.core.utils.get_field_data(result, "data")
        .and_then(|data| plugin.core.utils.get_field_data(data, "single_mode_load_common"))
        .and_then(|single_mode_load_common| plugin.core.utils.get_field_data(single_mode_load_common, "chara_info"))
        .and_then(|chara_info| plugin.core.utils.get_field_data(chara_info, "support_card_array"))
        .and_then(|support_card_array| plugin.core.utils.get_vector(support_card_array))
    else { return result };

    let mut cards = HashMap::new();
    for support_card_data in support_card_array_vector {
        // response > data > single_mode_load_common > chara_info > support_card_array[i] > support_card_id
        let Ok(id) = plugin.core.utils.get_field_data(support_card_data, "support_card_id")
        else { return result };

        let id = id as i64;

        // response > data > single_mode_load_common > chara_info > support_card_array[i] > position
        let Ok(position) = plugin.core.utils.get_field_data(support_card_data, "position")
        else { return result };

        let position = position as i64;

        let Some(support_card) = support_cards.get(&id)
        else { return result };

        let Some(name) = chara_names.get(&support_card.chara_id)
        else { continue };

        let name = name.resolve();
        let rarity = Rarity::from(support_card.rarity);
        let r#type = Type::from(support_card.r#type);

        cards.insert(id, (position, name, rarity, r#type));
    }

    
    let console = &plugin.console;
    let ui = &console.ui;
    
    ui.set(cards);
    console.update();
    
    result
}

pub unsafe fn init(image: &str, namespace: &str, class: &str) {
    let Some(plugin) = PLUGIN.get()
    else { return };

    plugin.core.hooks.install(
        image, namespace, class,
        "Deserialize", 4,
        single_mode_breeders_load_hook as Address
    );
}