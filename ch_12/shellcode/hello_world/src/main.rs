#![no_std]
#![no_main]
#![feature(asm)]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "x86_64")]
const SYS_WRITE: u64 = 1;
#[cfg(target_arch = "x86_64")]
const SYS_EXIT: u64 = 60;

#[cfg(target_arch = "aarch64")]
const SYS_WRITE: u64 = 64;
#[cfg(target_arch = "aarch64")]
const SYS_EXIT: u64 = 93;

const STDOUT: u64 = 1;
static MESSAGE: &str = "hello world\n";

#[no_mangle]
fn _start() {
    unsafe {
        syscalls::syscall3(
            SYS_WRITE,
            STDOUT,
            MESSAGE.as_ptr() as u64,
            MESSAGE.len() as u64,
        );

        syscalls::syscall1(SYS_EXIT, 0)
    };
}

mod syscalls {
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    /// x86_64
    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[cfg(target_arch = "x86_64")]
    pub unsafe fn syscall1(syscall: u64, arg1: u64) -> u64 {
        let ret: u64;
        asm!(
            "syscall",
            in("rax") syscall,
            in("rdi") arg1,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack),
        );
        ret
    }

    #[cfg(target_arch = "x86_64")]
    pub unsafe fn syscall3(syscall: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
        let ret: u64;
        asm!(
            "syscall",
            in("rax") syscall,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack),
        );
        ret
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////
    /// aarch64
    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[cfg(target_arch = "aarch64")]
    pub unsafe fn syscall1(syscall: u64, arg1: u64) -> u64 {
        let ret: u64;
        asm!(
            "svc 0",
            in("x8") syscall,
            in("x0") arg1,
            lateout("x0") ret,
            options(nostack),
        );
        ret
    }

    #[cfg(target_arch = "aarch64")]
    pub unsafe fn syscall3(syscall: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
        let ret: u64;
        asm!(
            "adr x1, .",
            "add x1, x1, #40",
            in("x4") arg2,
        );
        asm!(
            "svc 0",
            in("x8") syscall,
            in("x0") arg1,
            in("x2") arg3,
            lateout("x0") ret,
            options(nostack),
        );
        ret
    }
}
