fn main() {
    let exit_status: libc::c_int = libc::EXIT_SUCCESS;
    unsafe {
        libc::exit(exit_status);
    };
}
