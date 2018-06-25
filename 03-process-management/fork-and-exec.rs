extern crate nix;
use nix::libc::{EXIT_FAILURE, EXIT_SUCCESS};
use nix::unistd::{execve, fork, getpid, ForkResult, Pid};
use std::ffi::CString;
use std::process;

// child runs execve after recovery from fork called by parent process.
fn child() {
    let args = &[
        CString::new("/bin/echo").unwrap(),
        CString::new("hello").unwrap(),
    ];
    let envs = &[CString::new("").unwrap()];
    println!("I'm child! my pid is {}", getpid());
    execve(&CString::new("/bin/echo").unwrap(), args, envs).unwrap();
    process::exit(EXIT_FAILURE);
}

fn parent(pid_c: Pid) {
    println!(
        "I'm parent! my pid is {} and the pid of my child is {}.",
        getpid(),
        pid_c
    );
    process::exit(EXIT_SUCCESS);
}

fn main() {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => parent(child),
        Ok(ForkResult::Child) => child(),
        Err(_) => {
            println!("fork() failed");
            process::exit(EXIT_FAILURE);
        }
    }
}
