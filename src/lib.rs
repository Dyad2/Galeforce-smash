#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_macros)]
#![allow(unused_imports)]

mod fighters;
mod ecb_shifts;
mod edge_cancels;
mod func_hooks;
mod custom_vars;

#[skyline::main(name = "galeforce")]
pub fn main() {

    //code edits
    fighters::install();
    func_hooks::install();
    custom_vars::install();
    galeforce_utils::vars::install();
}