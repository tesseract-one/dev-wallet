use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Result;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub (super) fn init() -> Result<()> {
    let level = if cfg!(debug_assertions) {
        log::LogLevel::Debug
        //LogLevelNum::Trace
    } else {
        log::LogLevel::Error
    };

    if !INITIALIZED.swap(true, Ordering::Relaxed) {
        stderrlog::new().verbosity(level as usize)
            .module("DevWallet")
            .init()?;
    }
    log_panics::init();
    Ok(())
}