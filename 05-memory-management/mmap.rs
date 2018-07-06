extern crate nix;

use std::ffi::CString;
use std::ptr::null_mut;

use nix::libc::system;
use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};
use nix::unistd::{getpid, Pid};

const ALLOC_SIZE: usize = 100 * 1024 * 1024;

fn main() {
    let pid = getpid();

    println!("*** memory map before memory allocation ***");
    show_mmap(&pid);

    let new_memory = unsafe {
        mmap(
            null_mut(),
            ALLOC_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS,
            -1,
            0,
        ).expect("mmap() failed")
    };

    println!("");
    println!(
        "*** succeeded to allocate memory: address = {:?}; size = {:#x} ***",
        new_memory, ALLOC_SIZE
    );

    println!("");
    println!("*** memory map after memory allocation ***");
    show_mmap(&pid);

    unsafe { munmap(new_memory, ALLOC_SIZE).expect("munmap() failed") };
}

fn show_mmap(pid: &Pid) {
    let command = CString::new(format!("cat /proc/{}/maps", pid)).unwrap();
    unsafe { system(command.into_raw()) };
}
