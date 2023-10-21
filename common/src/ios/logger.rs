use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Result;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub (super) fn init() -> Result<()> {
    let level = if cfg!(debug_assertions) {
        stderrlog::LogLevelNum::Debug
    } else {
        stderrlog::LogLevelNum::Error
    };

    if !INITIALIZED.swap(true, Ordering::Relaxed) {
        stderrlog::new().verbosity(level)
            .module("DevWallet")
            .show_module_names(true)
            .init()?;
    }
    log_panics::init();
    Ok(())
}