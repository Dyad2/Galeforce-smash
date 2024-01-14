#![feature(
    concat_idents,
    proc_macro_hygiene,
    simd_ffi,
    repr_simd
)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_macros)]

use skyline::libc::c_char;

mod fighter;
mod func_hooks;
mod custom_vars;

extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {
        let version = env!("CARGO_PKG_VERSION").to_string();
        let new_str = format!(
            "{} - Galeforce Ver. {}\0",
            original_str,
            version
        );
        call_original!(arg, skyline::c_str(&new_str))
    }
    else {
        call_original!(arg, string)
    }
}

#[skyline::main(name = "galeforce")]
pub fn main() {
    
    println!("[Galeforce_sl2::main] Loading");
    //code edits
    fighter::install();
    func_hooks::install();
    custom_vars::install();
    galeforce_utils::vars::install();
    skyline::install_hooks!(change_version_string_hook);
    println!("[Galeforce_sl2::main] Loaded!");
}