extern crate nix;
extern crate time;

use std::ffi::CString;
use std::process;
use std::ptr::write;

use nix::libc::{malloc, system, EXIT_FAILURE, EXIT_SUCCESS};
use nix::sys::wait::wait;
use nix::unistd::{fork, getpid, ForkResult};

const BUFFER_SIZE: usize = 100 * 1024 * 1024;
const PAGE_SIZE: usize = 4096;

fn child_fn(p: *mut i8) {
    println!("*** child ps info before memory access ***");

    ps();

    println!("*** free memory info before memory access ***");
    free();

    for i in (0..BUFFER_SIZE).step_by(PAGE_SIZE) {
        unsafe { write(p.add(i), 0) };
    }

    println!("*** child ps info after memory access");
    ps();

    println!("*** free memory info after memory access ***");
    free();

    process::exit(EXIT_SUCCESS);
}

fn parent_fn() {
    let _ = wait();
    process::exit(EXIT_SUCCESS);
}

fn ps() {
    let pid = getpid();
    let command = CString::new(format!(
        "ps -o pid,comm,vsz,rss,min_flt,maj_flt | grep {}",
        pid
    )).unwrap();
    unsafe { system(command.into_raw()) };
}

fn free() {
    let command = CString::new("free").unwrap();
    unsafe { system(command.into_raw()) };
}

fn main() {
    let p = unsafe { malloc(BUFFER_SIZE) } as *mut i8;

    for i in (0..BUFFER_SIZE).step_by(PAGE_SIZE) {
        unsafe { write(p.add(i), 0) };
    }

    println!("*** free memory info before fork ***");

    free();

    match fork() {
        Ok(ForkResult::Parent { child: _ }) => parent_fn(),
        Ok(ForkResult::Child) => child_fn(p),
        Err(_) => {
            println!("fork() failed");
            process::exit(EXIT_FAILURE);
        }
    }
}
