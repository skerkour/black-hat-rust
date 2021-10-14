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

#[cfg(target_arch = "aarch64")]
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
