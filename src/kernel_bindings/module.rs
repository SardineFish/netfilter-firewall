use super::bindings;

extern crate concat_idents;
extern crate proc_macros;

extern "C" {
    #[no_mangle]
    static mut __this_module: bindings::module ;
}

pub fn this_module() -> *mut bindings::module {
    unsafe { return &mut __this_module; }
}

#[macro_export]
macro_rules! module_init {
    ($name: ident) => {
        fn __invoke_init_func(f: fn() -> i32) -> i32 {
            return f();
        }
        #[no_mangle]
        pub extern "C" fn init_module() -> i32 {
            return __invoke_init_func($name);
        }
    };
}

#[macro_export]
macro_rules! module_exit {
    ($name: ident) => {
        #[no_mangle]
        pub extern "C" fn cleanup_module() {
            $name();
        }
    };
}

#[macro_export]
macro_rules! mod_info {
    ($key:ident, $value:expr) => {
        concat_idents::concat_idents!(val_name = __KEY_, $key, {
            #[link_section = ".modinfo"]
            #[used]
            #[allow(non_upper_case_globals)]
            pub static val_name: [u8; stringify!($key).len()] = *proc_macros::stringify_to_bytes!($key);
        });
        concat_idents::concat_idents!(val_name = __SEP_, $key, {
            #[link_section = ".modinfo"]
            #[used]
            #[allow(non_upper_case_globals)]
            pub static val_name: [u8; 1] = *b"=";
        });
        concat_idents::concat_idents!(val_name = __VALUE_, $key, {
            #[link_section = ".modinfo"]
            #[used]
            #[allow(non_upper_case_globals)]
            pub static val_name: [u8; $value.len()] = *proc_macros::str_u8_array!($value);
        });
        concat_idents::concat_idents!(val_name = __NUL_, $key, {
            #[link_section = ".modinfo"]
            #[used]
            #[allow(non_upper_case_globals)]
            pub static val_name: [u8; 1] = *b"\0";
        });
    };
}

#[macro_export]
macro_rules! module_license {
    ($license: expr) => {
        $crate::mod_info!(license, $license);
    };
}

#[macro_export]
macro_rules! module_author {
    ($author: expr) => {
        $crate::mod_info!(author, $author);
    };
}

#[macro_export]
macro_rules! module_description {
    ($description: expr) => {
        $crate::mod_info!(description, $description);
    };
}

#[macro_export]
macro_rules! module_version {
    ($version: expr) => {
        $crate::mod_info!(version, $version);
    };
}