#![feature(duration_as_u128)]

use std::env;
use std::mem::uninitialized;
use std::process::exit;
use std::time::Duration;

extern crate nix;
use nix::libc::{clock_gettime, timespec, CLOCK_MONOTONIC, EXIT_FAILURE, EXIT_SUCCESS};
use nix::sys::signal::{kill, Signal};
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult};

const NLOOP_FOR_ESTIMATION: u64 = 50_000_000;

#[inline]
fn diff_msec(before: &timespec, after: &timespec) -> u128 {
    Duration::new(
        (after.tv_sec - before.tv_sec) as u64,
        (after.tv_nsec - before.tv_nsec) as u32,
    ).as_millis()
}

fn get_cpu_time() -> timespec {
    let mut tp: timespec = unsafe { uninitialized() };
    unsafe { clock_gettime(CLOCK_MONOTONIC, &mut tp) };
    tp
}

fn loops_per_msec() -> u64 {
    let before: timespec = get_cpu_time();
    for _i in 0..NLOOP_FOR_ESTIMATION {}
    let after: timespec = get_cpu_time();
    return NLOOP_FOR_ESTIMATION / (diff_msec(&before, &after) as u64);
}

#[inline]
fn load(nloop: u64) {
    for _i in 0..nloop {}
}

fn child_fn(
    id: u32,
    buf: &mut Vec<timespec>,
    nrecord: u32,
    nloop_per_resol: u64,
    start: &timespec,
) {
    for _i in 0..nrecord {
        load(nloop_per_resol);
        let ts = get_cpu_time();
        buf.push(ts);
    }
    for i in 0..nrecord {
        println!(
            "{}\t{}\t{}",
            id,
            diff_msec(start, &buf[i as usize]),
            (i + 1) * 100 / nrecord
        )
    }
    exit(EXIT_SUCCESS);
}

fn main() {
    let args: Vec<u32> = env::args()
        .map(|s| s.parse())
        .filter_map(|r| r.ok())
        .collect();

    if args.len() < 3 {
        eprintln!(
            "usage: {} <nproc> <total [ms]> <resolution[ms]>\n",
            env::args().nth(0).unwrap(),
        );
        exit(EXIT_FAILURE);
    }

    let nproc = args[0];
    let total = args[1];
    let resol = args[2];

    if nproc < 1 {
        eprintln!("<nproc>({}) should be >= 1", nproc);
        exit(EXIT_FAILURE);
    }

    if total < 1 {
        eprintln!("<resol>({}) should be >= 1", total);
        exit(EXIT_FAILURE);
    }

    if resol < 1 {
        eprintln!("<resol>({}) should be >= 1", resol);
        exit(EXIT_FAILURE);
    }

    if total % resol != 0 {
        eprintln!(
            "<total>({}) should be mulitiple of <resolution>({})",
            total, resol
        );
        exit(EXIT_FAILURE);
    }
    let nrecord = total / resol;

    let mut logbuf = Vec::with_capacity(nrecord as usize);
    println!("estimating workload which takes just one milisecond");
    let nloop_per_resol = loops_per_msec() * (resol as u64);
    println!("end estimation");

    let mut pids = Vec::with_capacity(nproc as usize);

    let start = get_cpu_time();
    let mut ncreated = 0;
    for i in 0..nproc {
        match fork() {
            Ok(ForkResult::Parent { child }) => {
                pids.push(child);
                ncreated += 1;
            }
            Ok(ForkResult::Child) => {
                child_fn(i, &mut logbuf, nrecord, nloop_per_resol, &start);
            }
            Err(_) => {
                eprintln!("fork() failed");
                for j in 0..ncreated {
                    if let Err(_) = kill(pids[j as usize], Signal::SIGINT) {
                        eprintln!("kill({}) failed", pids[j as usize]);
                        break;
                    }
                }
                exit(EXIT_FAILURE);
            }
        }
    }

    for _i in 0..ncreated {
        if let Err(_) = wait() {
            eprintln!("wait() failed");
        }
    }

    exit(EXIT_SUCCESS);
}
