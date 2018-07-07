extern crate nix;

use std::ffi::{CStr, CString};
use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;
use std::ptr::null_mut;

use nix::libc::{c_char, c_void, memcpy, strlen, system};
use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};
use nix::unistd::{getpid, Pid};

const ALLOC_SIZE: usize = 100 * 1024 * 1024;

fn main() {
    let overwrite_data = "HELLO";
    let overwite_data_ptr = CString::new(overwrite_data).unwrap().into_raw();

    let pid = getpid();

    println!("**memory map before mapping file ***");
    show_mmap(&pid);

    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open("testfile")
        .expect("open() failed");

    let file_contents_ptr = unsafe {
        mmap(
            null_mut(),
            ALLOC_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            fd.into_raw_fd(),
            0,
        ).expect("mmap() failed")
    };
    let file_contents = unsafe { CStr::from_ptr(file_contents_ptr as *mut c_char) }
        .to_str()
        .expect("to_str() failed");

    println!("");
    println!(
        "*** succeeded to map file: address = {:?}, size = 0x{:?} ***",
        file_contents_ptr, ALLOC_SIZE
    );
    show_mmap(&pid);

    println!("");
    println!(
        "*** file contents before overwite mapped region: {}",
        file_contents
    );

    // overwrite mapped region
    unsafe {
        memcpy(
            file_contents_ptr,
            overwite_data_ptr as *mut c_void,
            strlen(overwite_data_ptr),
        )
    };

    println!("");
    println!("*** overwritten mapped region with: {}", file_contents);

    unsafe { munmap(file_contents_ptr, ALLOC_SIZE).expect("munmap() failed") };
}

fn show_mmap(pid: &Pid) {
    let command = CString::new(format!("cat /proc/{}/maps", pid)).unwrap();
    unsafe { system(command.into_raw()) };
}
