use core::fmt;
use std::fmt::Display;

use tracing_subscriber::fmt::format::Writer;

pub(crate) const COLOR_RED: &str = "\x1b[31m";
pub(crate) const COLOR_GREEN: &str = "\x1b[32m";
pub(crate) const COLOR_YELLOW: &str = "\x1b[33m";
pub(crate) const COLOR_BLUE: &str = "\x1b[0;34m";
pub(crate) const COLOR_MAGENTA: &str = "\x1b[35m";
pub(crate) const COLOR_CYAN: &str = "\x1b[36m";
pub(crate) const COLOR_GRAY: &str = "\x1b[37m";
pub(crate) const COLOR_RESET: &str = "\x1b[0m";

pub(crate) trait ColorWriter {
    fn set_color(&mut self, color: &'static str) -> fmt::Result;
    fn reset_color(&mut self) -> fmt::Result;
    fn write_with_color(&mut self, content: impl Display, color: &'static str) -> fmt::Result;
}

impl ColorWriter for Writer<'_> {
    fn set_color(&mut self, color: &'static str) -> fmt::Result {
        if self.has_ansi_escapes() {
            self.write_str(color)?;
        }
        Ok(())
    }

    fn reset_color(&mut self) -> fmt::Result {
        if self.has_ansi_escapes() {
            self.write_str(COLOR_RESET)?;
        }
        Ok(())
    }

    fn write_with_color(&mut self, content: impl Display, color: &'static str) -> fmt::Result {
        if self.has_ansi_escapes() {
            write!(self, "{color}{content}{COLOR_RESET}")
        } else {
            write!(self, "{content}")
        }
    }
}
