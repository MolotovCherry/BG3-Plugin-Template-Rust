use std::panic;

use log::error;

#[cfg(debug_assertions)]
use crate::backtrace::CaptureBacktrace;

pub fn set_hook() {
    panic::set_hook(Box::new(move |info| {
        #[allow(unused_assignments)]
        let mut message = info.to_string();

        // For debug mode, print entire stack trace. Stack trace doesn't really
        // contain anything useful in release mode due to optimizations,
        // but you can enable it in release too if you want
        #[cfg(debug_assertions)]
        {
            message = format!("{info}\n\nstack backtrace:\n{}", CaptureBacktrace);
        }

        // Dump panic info to logfile
        error!("{message}");
    }));
}