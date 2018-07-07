extern crate nix;
extern crate time;

use std::process;
use std::ptr::write;

use nix::libc::EXIT_SUCCESS;
use nix::libc::{getchar, malloc};
use nix::unistd::sleep;

const BUFFER_SIZE: usize = 100 * 1024 * 1024;
const PAGE_SIZE: usize = 4096;
const NCYCLE: usize = 10;

fn main() {
    let t = time::now();
    println!("{}: before allocation, please press Enter key", t.ctime());
    unsafe { getchar() };

    let p = unsafe { malloc(BUFFER_SIZE) } as *mut i8;

    let t = time::now();
    println!(
        "{}: allocated {}MB, please press Enter key",
        t.ctime(),
        BUFFER_SIZE / (1024 * 1024)
    );

    for i in (0..BUFFER_SIZE).step_by(PAGE_SIZE) {
        unsafe { write(p.add(i), 0) };

        let cycle = i / (BUFFER_SIZE / NCYCLE);
        if cycle != 0 && i % (BUFFER_SIZE / NCYCLE) == 0 {
            let t = time::now();
            println!("{}: touched {}MB", t.ctime(), i / (1024 * 1024));
            sleep(1);
        }
    }

    let t = time::now();
    println!(
        "{}: touched {}, please press Enter key",
        t.ctime(),
        BUFFER_SIZE / (1024 * 1024)
    );

    unsafe { getchar() };

    process::exit(EXIT_SUCCESS);
}
