extern crate nix;
use nix::libc::{EXIT_FAILURE, EXIT_SUCCESS};
use nix::unistd::{fork, getpid, ForkResult, Pid};
use std::process;

// child prints out the pid of child process.
fn child() {
    println!("I'm child! my pid is {}", getpid());
    process::exit(EXIT_SUCCESS);
}

// parent prints out the pid of parent as well as child process passed by argument.
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
    println!("shouldn't reach here");
    process::exit(EXIT_FAILURE);
}
