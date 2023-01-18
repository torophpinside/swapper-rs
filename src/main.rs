use std::process::Command;
use sysinfo::SystemExt;

const MINIMAL_MEMORY_SIZE: u64 = 1024 * 1024 * 2;

fn main() {
    let mut system = sysinfo::System::new();

    system.refresh_all();

    let available_mem = system.available_memory();
    let used_swap = system.used_swap();

    if used_swap > 0u64 {
        run_clear_cache();
    }

    if available_mem > (used_swap + MINIMAL_MEMORY_SIZE) && used_swap > 0u64 {
        run_clear_swap();
    }
}

fn run_clear_cache() {
    let mut output = Command::new("/usr/bin/sync")
        .output()
        .expect("failed to execute sync command");

    println!("{:?}", output.stdout);
    output = Command::new("/bin/sh")
        .arg("-c")
        .arg("echo 3 > /proc/sys/vm/drop_caches")
        .output()
        .expect("failed to execute process");
    println!("{:?}", output.stdout);
}

fn run_clear_swap() {
    let mut output = Command::new("/sbin/swapoff")
        .arg("-a")
        .output()
        .expect("failed to execute swapoff");
    println!("{:?}", output.stdout);
    output = Command::new("/sbin/swapon")
        .arg("-a")
        .output()
        .expect("failed to execute swapon");
    println!("{:?}", output.stdout);
}
