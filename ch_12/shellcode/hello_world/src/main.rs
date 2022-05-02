#![no_std]
#![no_main]
#![feature(const_raw_ptr_deref)]

use core::arch::asm;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "x86_64")]
const SYS_WRITE: usize = 1;
#[cfg(target_arch = "x86_64")]
const SYS_EXIT: usize = 60;

#[cfg(target_arch = "aarch64")]
const SYS_WRITE: usize = 64;
#[cfg(target_arch = "aarch64")]
const SYS_EXIT: usize = 93;

#[cfg(target_arch = "arm")]
const SYS_WRITE: usize = 4;
#[cfg(target_arch = "arm")]
const SYS_EXIT: usize = 1;

const STDOUT: usize = 1;
static MESSAGE: &str = "hello world\n";

#[no_mangle]
fn _start() {
    unsafe {
        syscalls::syscall3(SYS_WRITE, STDOUT, MESSAGE.as_ptr() as usize, MESSAGE.len());
        syscalls::syscall1(SYS_EXIT, 0)
    };
}

mod syscalls {
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    // x86_64
    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[cfg(target_arch = "x86_64")]
    pub unsafe fn syscall1(syscall: usize, arg1: usize) -> usize {
        let ret: usize;
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
    pub unsafe fn syscall3(syscall: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
        let ret: usize;
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
    // aarch64
    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[cfg(target_arch = "aarch64")]
    pub unsafe fn syscall1(syscall: usize, arg1: usize) -> usize {
        let ret: usize;
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
    pub unsafe fn syscall3(syscall: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
        let ret: usize;
        asm!(
            "adr x1, .",
            "add x1, x1, #40",
            in("x4") arg2,
        );

        // asm!(
        //     "adr x1, {}",
        //     "sub x1, x1, 16",
        //     sym super::MESSAGE,
        // );

        asm!(
            "svc 0",
            in("x8") syscall,
            in("x0") arg1,
            // in("x1") arg2,
            in("x2") arg3,
            lateout("x0") ret,
            options(nostack),
        );
        ret
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////
    // arm
    ////////////////////////////////////////////////////////////////////////////////////////////////////

    // #[cfg(target_arch = "arm")]
    // pub unsafe fn syscall1(syscall: usize, arg1: usize) -> usize {
    //     let ret: u32;
    //     asm!(
    //         "svc 0",
    //         in("r7") syscall as u32,
    //         in("r0") arg1 as u32,
    //         lateout("r0") ret,
    //         options(nostack),
    //     );
    //     ret as usize
    // }

    // #[cfg(target_arch = "arm")]
    // pub unsafe fn syscall3(syscall: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    //     let ret: u32;
    //     asm!(
    //         "svc 0",
    //         in("r7") syscall as u32,
    //         in("r0") arg1 as u32,
    //         in("r1") arg2 as u32,
    //         in("r2") arg3 as u32,
    //         lateout("r0") ret,
    //         options(nostack),
    //     );
    //     ret as usize
    // }
}
