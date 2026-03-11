use crate::types::*;

use std::ffi::{c_char, c_void};

pub static mut VTABLE: Option<&'static Vtable> = None;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vtable {
    pub hachimi_instance: unsafe extern "C" fn() -> *const Hachimi,
    pub hachimi_get_interceptor: unsafe extern "C" fn(this: *const Hachimi) -> *const Interceptor,

    pub interceptor_hook: unsafe extern "C" fn(
        this: *const Interceptor,
        orig_addr: *mut MethodAddress,
        hook_addr: *mut c_void,
    ) -> usize,
    pub interceptor_hook_vtable: unsafe extern "C" fn(
        this: *const Interceptor,
        vtable: *mut *mut c_void,
        vtable_index: usize,
        hook_addr: *mut c_void,
    ) -> usize,
    pub interceptor_get_trampoline_addr:
        unsafe extern "C" fn(this: *const Interceptor, hook_addr: *mut c_void) -> usize,
    pub interceptor_unhook:
        unsafe extern "C" fn(this: *const Interceptor, hook_addr: *mut c_void) -> usize,
    pub list_free:
        unsafe extern "C" fn(ptr: *mut usize, size: usize),

    pub il2cpp_resolve_symbol: unsafe extern "C" fn(name: *const c_char) -> *mut c_void,
    pub il2cpp_get_assembly_image: unsafe extern "C" fn(assembly_name: *const c_char) -> *const Image,
    pub il2cpp_get_image_from_class: unsafe extern "C" fn(class: *mut Class) -> *const Image,
    pub il2cpp_get_image_name: unsafe extern "C" fn(image: *const Image) -> *const c_char,
    pub il2cpp_get_namespace_from_class: unsafe extern "C" fn(class: *mut Class) -> *const c_char,
    pub il2cpp_get_classes: unsafe extern "C" fn(
        image: *const Image
    ) -> List<*mut Class>,
    pub il2cpp_get_class: unsafe extern "C" fn(
        image: *const Image,
        namespace: *const c_char,
        class_name: *const c_char,
    ) -> *mut Class,
    pub il2cpp_get_class_from_method: unsafe extern "C" fn(
        method: *mut MethodInfo
    ) -> *mut Class,
    pub il2cpp_get_class_name: unsafe extern "C" fn(
        class: *mut Class
    ) -> *const c_char,
    pub il2cpp_get_methods: unsafe extern "C" fn(
        class: *const Class
    ) -> List<*mut MethodInfo>,
    pub il2cpp_get_method: unsafe extern "C" fn(
        class: *mut Class,
        name: *const c_char,
        args_count: i32,
    ) -> *const MethodInfo,
    pub il2cpp_get_method_name: unsafe extern "C" fn(
        method: *mut MethodInfo
    ) -> *const c_char,
    pub il2cpp_get_method_overload: unsafe extern "C" fn(
        class: *mut Class,
        name: *const c_char,
        params: *const TypeEnum,
        param_count: usize,
    ) -> *const MethodInfo,
    pub il2cpp_get_method_addr: unsafe extern "C" fn(
        class: *mut Class,
        name: *const c_char,
        args_count: i32,
    ) -> *mut MethodAddress,
    pub il2cpp_get_method_overload_addr: unsafe extern "C" fn(
        class: *mut Class,
        name: *const c_char,
        params: *const TypeEnum,
        param_count: usize,
    ) -> *mut MethodAddress,
    pub il2cpp_get_method_cached: unsafe extern "C" fn(
        class: *mut Class,
        name: *const c_char,
        args_count: i32,
    ) -> *const MethodInfo,
    pub il2cpp_get_method_addr_cached: unsafe extern "C" fn(
        class: *mut Class,
        name: *const c_char,
        args_count: i32,
    ) -> *mut MethodInfo,
    pub il2cpp_get_method_param_count: unsafe extern "C" fn(
        method: *mut MethodInfo
    ) -> u32,
    pub il2cpp_find_nested_class:
        unsafe extern "C" fn(class: *mut Class, name: *const c_char) -> *mut Class,
    pub il2cpp_get_field_from_name:
        unsafe extern "C" fn(class: *mut Class, name: *const c_char) -> *mut Field,
    pub il2cpp_get_field_value: unsafe extern "C" fn(
        obj: *mut Object,
        field: *mut Field,
        out_value: *mut c_void,
    ),
    pub il2cpp_set_field_value: unsafe extern "C" fn(
        obj: *mut Object,
        field: *mut Field,
        value: *const c_void,
    ),
    pub il2cpp_get_static_field_value:
        unsafe extern "C" fn(field: *mut Field, out_value: *mut c_void),
    pub il2cpp_set_static_field_value:
        unsafe extern "C" fn(field: *mut Field, value: *const c_void),
    pub il2cpp_unbox: unsafe extern "C" fn(obj: *mut Object) -> *mut c_void,
    pub il2cpp_get_main_thread: unsafe extern "C" fn() -> *mut Thread,
    pub il2cpp_get_attached_threads:
        unsafe extern "C" fn(out_size: *mut usize) -> *mut *mut Thread,
    pub il2cpp_schedule_on_thread:
        unsafe extern "C" fn(thread: *mut Thread, callback: unsafe extern "C" fn()),
    pub il2cpp_create_array:
        unsafe extern "C" fn(element_type: *mut Class, length: usize) -> *mut Array,
    pub il2cpp_get_singleton_like_instance:
        unsafe extern "C" fn(class: *mut Class) -> *mut Object,

    pub log: unsafe extern "C" fn(level: i32, target: *const c_char, message: *const c_char),
    pub gui_register_menu_item: unsafe extern "C" fn(
        label: *const c_char,
        callback: Option<extern "C" fn(*mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_register_menu_section: unsafe extern "C" fn(
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_show_notification: unsafe extern "C" fn(message: *const c_char) -> bool,
    pub gui_ui_heading: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_label: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_small: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_separator: unsafe extern "C" fn(ui: *mut c_void) -> bool,
    pub gui_ui_button: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_small_button: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_checkbox:
        unsafe extern "C" fn(ui: *mut c_void, text: *const c_char, value: *mut bool) -> bool,
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
        id: *const c_char,
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
        text: *const c_char,
    ) -> bool,
    pub gui_register_menu_item_icon: unsafe extern "C" fn(
        label: *const c_char,
        icon_uri: *const c_char,
        icon_ptr: *const u8,
        icon_len: usize,
    ) -> bool,
    pub gui_register_menu_section_with_icon: unsafe extern "C" fn(
        title: *const c_char,
        icon_uri: *const c_char,
        icon_ptr: *const u8,
        icon_len: usize,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,

    pub android_dex_load: unsafe extern "C" fn(
        dex_ptr: *const u8,
        dex_len: usize,
        class_name: *const c_char,
    ) -> u64,
    pub android_dex_unload: unsafe extern "C" fn(handle: u64) -> bool,
    pub android_dex_call_static_noargs:
        unsafe extern "C" fn(handle: u64, method: *const c_char, sig: *const c_char) -> bool,
    pub android_dex_call_static_string: unsafe extern "C" fn(
        handle: u64,
        method: *const c_char,
        sig: *const c_char,
        arg: *const c_char,
    ) -> bool,
}

impl Vtable {
    pub unsafe fn init(vtable: *const Vtable) -> &'static Vtable {
        VTABLE = Some(&*vtable);
        VTABLE.unwrap()
    }
}