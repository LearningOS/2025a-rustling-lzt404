//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", secs);
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
