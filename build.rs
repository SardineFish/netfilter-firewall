extern crate bindgen;

use std::env;

const INCLUDE_FUNC: &[&str] = &[
    "printk",
    "__netlink_kernel_create",
    "skb_dequeue",
    "nlmsg_data_non_inline",
    "nlmsg_new_non_inline",
    "netlink_cb",
    "netlink_unicast",
    "netlink_broadcast",
    "netlink_kernel_release",
    "nlmsg_total_size",
    "kmalloc_wrapped",
    "kcalloc_wrapped",
    "kfree_wrapped"
];
const INCLUDE_TYPE: &[&str] = &[
    "nlmsghdr",
    "module",
    "sk_buff",
    "GFP"
];
const INCLUDE_VAL: &[&str] = &[
];

fn path_join(base_path:&str, sub_path: &str) -> String {
    let stash = if base_path.ends_with("/") {""} else {"/"};
    if sub_path.starts_with("./") {
        return [base_path, stash, &sub_path[2..]].join("");
    }
    else if sub_path.starts_with("/") {
        return sub_path.to_string();
    }
    
    return [base_path, stash, sub_path].join("");
}

macro_rules! path_join {
    ($path:expr) => ($path);
    ($base:expr, $($rest:expr), +) => (path_join($base, path_join!($($rest), +)));
}


pub fn include_abspath(includes: &str, kernel_dir: &str) -> Vec<String> {
    let mut parts = includes.split_whitespace();
    let mut abspaths = vec![];
    while let Some(arg) = parts.next() {
        if arg.starts_with("-I") && !arg.starts_with("-I/") {
            abspaths.push(format!("-I{}", path_join!(kernel_dir, &arg[2..])));
        } 
        else if arg == "-include" {
            abspaths.push("-include".to_string());
            abspaths.push(path_join!(kernel_dir, parts.next().unwrap()));
            // abspaths.push(format!("-include {}", path_join!(kernel_dir, parts.next().unwrap())));
        }
        else {
            abspaths.push(arg.to_string());
        }
    }

    return abspaths;
}


const KBUILD_ENV_ERROR: &str = "Must invoke from Kbuild.";

pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/kernel_bindings/wrapper.h");
    println!("cargo:rerun-if-changed=src/kernel_bindings/helper.h");


    let kbuild_cflags_module = env::var("KBUILD_CFLAGS_MODULE").expect(KBUILD_ENV_ERROR);
    let kbuild_cppflags = env::var("KBUILD_CPPFLAGS").expect(KBUILD_ENV_ERROR);
    let kernel_dir = env::var("KDIR").expect(KBUILD_ENV_ERROR);
    let base_dir = env::var("BASE_DIR").expect(KBUILD_ENV_ERROR);

    let override_include = path_join!(&base_dir, "src/kernel_bindings/include");

    let linux_include = env::var("LINUXINCLUDE")
        .expect("Need be invoked from Kbuild.");

    let mut binding = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("c_types")
        .derive_default(true)
        .size_t_is_usize(true)
        .rustfmt_bindings(true)
        .header("src/kernel_bindings/wrapper.h")
        .clang_arg(format!("-I{}", override_include))
        .clang_args(kbuild_cflags_module.split_whitespace())
        .clang_args(kbuild_cppflags.split_whitespace())
        .clang_args(include_abspath(&linux_include, &kernel_dir));

    for func in INCLUDE_FUNC {
        binding = binding.whitelist_function(func);
    }

    for type_to_include in INCLUDE_TYPE {
        binding = binding.whitelist_type(type_to_include);
    }

    for val in INCLUDE_VAL {
        binding = binding.whitelist_var(val);
    }
    
    binding
        .generate()
        .expect("Failed to generate bindings.")
        .write_to_file("src/kernel_bindings/c_bindings.rs")
        .expect("Failed to write file.");
}