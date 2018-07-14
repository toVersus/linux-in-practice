#![feature(duration_as_u128)]

extern crate nix;

use std::env;
use std::mem::uninitialized;
use std::process::exit;
use std::ptr::null_mut;
use std::ptr::write;
use std::time::Duration;

use nix::libc::{clock_gettime, timespec, CLOCK_MONOTONIC, EXIT_FAILURE, EXIT_SUCCESS};
use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};

const CACHE_LINE_SIZE: usize = 64;
const NLOOP: usize = 4 * 1024 * 1024 * 1024;

#[inline]
fn diff_nsec(before: &timespec, after: &timespec) -> u128 {
    Duration::new(
        (after.tv_sec - before.tv_sec) as u64,
        (after.tv_nsec - before.tv_nsec) as u32,
    ).as_nanos()
}

fn get_cpu_time() -> timespec {
    let mut tp: timespec = unsafe { uninitialized() };
    unsafe { clock_gettime(CLOCK_MONOTONIC, &mut tp) };
    tp
}

fn main() {
    let args: Vec<usize> = env::args()
        .map(|s| s.parse())
        .filter_map(|r| r.ok())
        .collect();

    if args.len() != 1 {
        eprintln!("usage: {} <size[KB]>\n", env::args().nth(0).unwrap());
        exit(EXIT_FAILURE);
    }

    let size: usize = args[0] * 1024;
    if size == 0 {
        eprintln!("size should be >= 1: {}", size);
        exit(EXIT_FAILURE);
    }

    let buffer = unsafe {
        mmap(
            null_mut(),
            size,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS,
            -1,
            0,
        ).expect("mmap() failed")
    };

    let before = get_cpu_time();

    for _i in 0..(NLOOP / (size / CACHE_LINE_SIZE)) {
        for j in (0..size).step_by(CACHE_LINE_SIZE) {
            unsafe { write((buffer as *mut i8).add(j), 0) };
        }
    }

    let after = get_cpu_time();

    println!("{}", (diff_nsec(&before, &after) as f64) / NLOOP as f64);

    unsafe { munmap(buffer, size).expect("munmap() failed") };
    exit(EXIT_SUCCESS);
}
