#![no_std]
#![no_main]
#![feature(asm)]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod syscalls;

#[cfg(target_arch = "x86_64")]
const SYS_WRITE: u64 = 1;
#[cfg(target_arch = "x86_64")]
const SYS_EXIT: u64 = 60;

#[cfg(target_arch = "aarch64")]
const SYS_WRITE: u64 = 64;
#[cfg(target_arch = "aarch64")]
const SYS_EXIT: u64 = 93;

const STDOUT: u64 = 1;

#[no_mangle]
fn _start() {
    unsafe {
        let message: &str = "hello world\n";
        let message_ptr = message.as_ptr() as u64;
        syscalls::syscall3(
            SYS_WRITE,
            STDOUT,
            message_ptr,
            message.len() as u64,
        );

        syscalls::syscall1(SYS_EXIT, 0)
    };
}
