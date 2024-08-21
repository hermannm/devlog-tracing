use core::fmt;

use chrono::Local;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

#[derive(Default)]
pub struct DevLogTimeFormat {
    // Prevents direct struct initialization, so we can add fields here later as a non-breaking
    // change.
    _private: (),
}

impl FormatTime for DevLogTimeFormat {
    fn format_time(&self, writer: &mut Writer<'_>) -> fmt::Result {
        let time = Local::now();
        write!(writer, "[{}]", time.format("%H:%M:%S"))?;
        Ok(())
    }
}
