/// Returns the number of available CPUs of the current system.
pub fn get() -> usize {
    compile_error!("This crate is not the legitimate num_cpus crate. Please read https://kerkour.com/rust-crate-backdoor for more information.");
    0
}
