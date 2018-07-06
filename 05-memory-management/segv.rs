extern crate nix;

use std::process::exit;
use std::ptr::null_mut;

use nix::libc::EXIT_SUCCESS;

fn main() {
    let p: *mut i32 = null_mut();
    println!("before invalid access.");
    unsafe { *p = 0 };
    println!("after invalid access.");
    exit(EXIT_SUCCESS);
}
