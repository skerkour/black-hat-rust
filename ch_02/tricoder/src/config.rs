use std::time::Duration;

pub const SCAN_TIMEOUT: Duration = Duration::from_secs(3);
pub const HTTP_TIMEOUT: Duration = Duration::from_secs(30);
pub const RESOLUTION_TIMEOUT: Duration = Duration::from_secs(4);
pub const MAX_REDIRECTS: usize = 4;
pub const NUM_THREADS: usize = 256;
