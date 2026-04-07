use std::ffi::{c_char, c_void};

use crate::types::*;
use crate::il2cpp::types::*;

pub static mut VTABLE: Option<&'static Vtable> = None;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vtable {
    pub hachimi_instance: unsafe extern "C" fn() -> Hachimi,
    pub hachimi_get_interceptor: unsafe extern "C" fn(this: Hachimi) -> Interceptor,

    pub interceptor_hook: unsafe extern "C" fn(
        this: Interceptor,
        orig_addr: Address,
        hook_addr: Address,
    ) -> Address,
    pub interceptor_hook_vtable: unsafe extern "C" fn(
        this: Interceptor,
        vtable: Address,
        vtable_index: usize,
        hook_addr: Address,
    ) -> Address,
    pub interceptor_get_trampoline_addr:
        unsafe extern "C" fn(this: Interceptor, hook_addr: Address) -> Address,
    pub interceptor_unhook:
        unsafe extern "C" fn(this: Interceptor, hook_addr: Address) -> Address,

    pub il2cpp_resolve_symbol: unsafe extern "C" fn(name: String) -> *mut c_void,
    pub il2cpp_get_assembly_image:
        unsafe extern "C" fn(assembly_name: IString) -> IImage,
    pub il2cpp_get_class: unsafe extern "C" fn(
        image: IImage,
        namespace: String,
        class_name: String,
    ) -> IClass,
    pub il2cpp_get_method: unsafe extern "C" fn(
        class: IClass,
        name: String,
        args_count: i32,
    ) -> IMethod,
    pub il2cpp_get_method_overload: unsafe extern "C" fn(
        class: IClass,
        name: String,
        params: ITypeEnum,
        param_count: usize,
    ) -> IMethod,
    pub il2cpp_get_method_addr: unsafe extern "C" fn(
        class: IClass,
        name: String,
        args_count: i32,
    ) -> Address,
    pub il2cpp_get_method_overload_addr: unsafe extern "C" fn(
        class: IClass,
        name: String,
        params: ITypeEnum,
        param_count: usize,
    ) -> Address,
    pub il2cpp_get_method_cached: unsafe extern "C" fn(
        class: IClass,
        name: String,
        args_count: i32,
    ) -> IMethod,
    pub il2cpp_get_method_addr_cached: unsafe extern "C" fn(
        class: IClass,
        name: String,
        args_count: i32,
    ) -> Address,
    pub il2cpp_find_nested_class:
        unsafe extern "C" fn(class: IClass, name: String) -> IClass,
    pub il2cpp_get_field_from_name:
        unsafe extern "C" fn(class: IClass, name: String) -> IField,
    pub il2cpp_get_field_value: unsafe extern "C" fn(
        obj: IObject,
        field: IField,
        out_value: *mut c_void,
    ),
    pub il2cpp_set_field_value: unsafe extern "C" fn(
        obj: IObject,
        field: IField,
        value: *mut c_void,
    ),
    pub il2cpp_get_static_field_value:
        unsafe extern "C" fn(field: IField, out_value: *mut c_void),
    pub il2cpp_set_static_field_value:
        unsafe extern "C" fn(field: IField, value: *mut c_void),
    pub il2cpp_unbox: unsafe extern "C" fn(obj: IObject) -> *mut c_void,
    pub il2cpp_get_main_thread: unsafe extern "C" fn() -> IThread,
    pub il2cpp_get_attached_threads:
        unsafe extern "C" fn(out_size: *mut usize) -> *mut IThread,
    pub il2cpp_schedule_on_thread:
        unsafe extern "C" fn(thread: IThread, callback: unsafe extern "C" fn()),
    pub il2cpp_create_array:
        unsafe extern "C" fn(element_type: IClass, length: usize) -> IArray,
    pub il2cpp_get_singleton_like_instance:
        unsafe extern "C" fn(class: IClass) -> IObject,

    pub log: unsafe extern "C" fn(level: i32, target: *mut c_char, message: *mut c_char),
    pub gui_register_menu_item: unsafe extern "C" fn(
        label: *mut c_char,
        callback: Option<extern "C" fn(*mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_register_menu_section: unsafe extern "C" fn(
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_show_notification: unsafe extern "C" fn(message: *mut c_char) -> bool,
    pub gui_ui_heading: unsafe extern "C" fn(ui: *mut c_void, text: *mut c_char) -> bool,
    pub gui_ui_label: unsafe extern "C" fn(ui: *mut c_void, text: *mut c_char) -> bool,
    pub gui_ui_small: unsafe extern "C" fn(ui: *mut c_void, text: *mut c_char) -> bool,
    pub gui_ui_separator: unsafe extern "C" fn(ui: *mut c_void) -> bool,
    pub gui_ui_button: unsafe extern "C" fn(ui: *mut c_void, text: *mut c_char) -> bool,
    pub gui_ui_small_button: unsafe extern "C" fn(ui: *mut c_void, text: *mut c_char) -> bool,
    pub gui_ui_checkbox:
        unsafe extern "C" fn(ui: *mut c_void, text: *mut c_char, value: *mut bool) -> bool,
    pub gui_ui_text_edit_singleline: unsafe extern "C" fn(
        ui: *mut c_void,
        buffer: *mut c_char,
        buffer_len: usize,
    ) -> bool,
    pub gui_ui_horizontal: unsafe extern "C" fn(
        ui: *mut c_void,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_ui_grid: unsafe extern "C" fn(
        ui: *mut c_void,
        id: *mut c_char,
        columns: usize,
        spacing_x: f32,
        spacing_y: f32,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_ui_end_row: unsafe extern "C" fn(ui: *mut c_void) -> bool,
    pub gui_ui_colored_label: unsafe extern "C" fn(
        ui: *mut c_void,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        text: *mut c_char,
    ) -> bool,
    pub gui_register_menu_item_icon: unsafe extern "C" fn(
        label: *mut c_char,
        icon_uri: *mut c_char,
        icon_ptr: *const u8,
        icon_len: usize,
    ) -> bool,
    pub gui_register_menu_section_with_icon: unsafe extern "C" fn(
        title: *mut c_char,
        icon_uri: *mut c_char,
        icon_ptr: *const u8,
        icon_len: usize,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,

    pub android_dex_load: unsafe extern "C" fn(
        dex_ptr: *const u8,
        dex_len: usize,
        class_name: *mut c_char,
    ) -> u64,
    pub android_dex_unload: unsafe extern "C" fn(handle: u64) -> bool,
    pub android_dex_call_static_noargs:
        unsafe extern "C" fn(handle: u64, method: *mut c_char, sig: *mut c_char) -> bool,
    pub android_dex_call_static_string: unsafe extern "C" fn(
        handle: u64,
        method: *mut c_char,
        sig: *mut c_char,
        arg: *mut c_char,
    ) -> bool,
}

impl Vtable {
    pub unsafe fn init(vtable: *const Vtable) -> &'static Vtable {
        VTABLE = Some(&*vtable);
        VTABLE.unwrap()
    }
}