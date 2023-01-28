use std::sync::atomic::{AtomicBool, Ordering};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub (super) fn init() -> crate::error::Result<()> {
    use stderrlog::LogLevelNum;

    let level = if cfg!(debug_assertions) {
        LogLevelNum::Debug
        //LogLevelNum::Trace
    } else {
        LogLevelNum::Error
    };

    Ok(if !INITIALIZED.load(Ordering::Relaxed) {
        stderrlog::new().verbosity(level).show_module_names(true).init()?;
        INITIALIZED.store(true, Ordering::Relaxed)
    })
}