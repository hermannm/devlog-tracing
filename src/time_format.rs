use core::fmt;

use tracing_subscriber::fmt::{
    format::Writer,
    time::{ChronoLocal, FormatTime},
};

pub struct DevLogTimeFormat {
    inner: ChronoLocal,
}

impl Default for DevLogTimeFormat {
    fn default() -> Self {
        Self {
            inner: ChronoLocal::new("%H:%M:%S".to_owned()),
        }
    }
}

impl FormatTime for DevLogTimeFormat {
    #[inline]
    fn format_time(&self, writer: &mut Writer<'_>) -> fmt::Result {
        writer.write_char('[')?;
        self.inner.format_time(writer)?;
        writer.write_char(']')?;
        Ok(())
    }
}
