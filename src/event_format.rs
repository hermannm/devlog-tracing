use core::fmt;
use std::thread;

use crate::{
    color::{ColorWriter, COLOR_CYAN},
    field_format::DevLogFieldFormat,
    time_format::DevLogTimeFormat,
};

use super::color::{COLOR_BLUE, COLOR_GRAY, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_YELLOW};
use tracing::{Event, Level, Metadata};
use tracing_core::subscriber::Subscriber;
use tracing_subscriber::{
    field::VisitOutput,
    fmt::{format::Writer, time::FormatTime, FmtContext, FormatEvent, FormattedFields},
    registry::LookupSpan,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct DevLogEventFormat<TimeFormatT> {
    pub timer: TimeFormatT,
    pub display_timestamp: bool,
    pub display_target: bool,
    pub display_level: bool,
    pub display_thread_id: bool,
    pub display_thread_name: bool,
    pub display_filename: bool,
    pub display_line_number: bool,
}

impl Default for DevLogEventFormat<DevLogTimeFormat> {
    fn default() -> Self {
        Self::new()
    }
}

impl DevLogEventFormat<DevLogTimeFormat> {
    pub fn new() -> Self {
        Self {
            timer: DevLogTimeFormat::default(),
            display_timestamp: true,
            display_target: true,
            display_level: true,
            display_thread_id: false,
            display_thread_name: false,
            display_filename: false,
            display_line_number: false,
        }
    }
}

impl<SubscriberT, TimeFormatT> FormatEvent<SubscriberT, DevLogFieldFormat>
    for DevLogEventFormat<TimeFormatT>
where
    SubscriberT: Subscriber + for<'a> LookupSpan<'a>,
    TimeFormatT: FormatTime,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, SubscriberT, DevLogFieldFormat>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let metadata = event.metadata();

        self.format_timestamp(&mut writer)?;
        self.format_level(*metadata.level(), &mut writer)?;
        self.format_target(metadata, &mut writer)?;
        self.format_fields(ctx, &mut writer, event)?;
        self.format_spans(ctx, &mut writer)?;
        self.format_source_location(metadata, &mut writer)?;
        self.format_thread_info(&mut writer)?;

        writeln!(writer)
    }
}

impl<TimeFormatT> DevLogEventFormat<TimeFormatT> {
    const TRACE_STR: &'static str = "TRACE";
    const DEBUG_STR: &'static str = "DEBUG";
    const INFO_STR: &'static str = "INFO";
    const WARN_STR: &'static str = "WARN";
    const ERROR_STR: &'static str = "ERROR";

    fn format_timestamp(&self, writer: &mut Writer<'_>) -> fmt::Result
    where
        TimeFormatT: FormatTime,
    {
        if self.display_timestamp {
            writer.set_color(COLOR_GRAY)?;
            if self.timer.format_time(writer).is_err() {
                writer.write_str("<unknown time>")?;
            }
            writer.reset_color()?;

            writer.write_char(' ')?;
        }

        Ok(())
    }

    fn format_level(&self, level: Level, writer: &mut Writer<'_>) -> fmt::Result {
        if self.display_level {
            let (level_string, color) = match level {
                Level::TRACE => (Self::TRACE_STR, COLOR_MAGENTA),
                Level::DEBUG => (Self::DEBUG_STR, COLOR_BLUE),
                Level::INFO => (Self::INFO_STR, COLOR_GREEN),
                Level::WARN => (Self::WARN_STR, COLOR_YELLOW),
                Level::ERROR => (Self::ERROR_STR, COLOR_RED),
            };

            writer.write_with_color(level_string, color)?;
            writer.write_char(' ')?;
        }

        Ok(())
    }

    fn format_target(&self, metadata: &Metadata<'static>, writer: &mut Writer<'_>) -> fmt::Result {
        if self.display_target {
            writer.set_color(COLOR_GRAY)?;
            write!(writer, "{}:", metadata.target())?;
            writer.reset_color()?;
            writer.write_char(' ')?;
        }

        Ok(())
    }

    fn format_fields<SubscriberT>(
        &self,
        ctx: &FmtContext<'_, SubscriberT, DevLogFieldFormat>,
        writer: &mut Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result
    where
        SubscriberT: Subscriber + for<'a> LookupSpan<'a>,
    {
        let mut visitor = ctx.field_format().make_field_visitor(writer.by_ref());
        event.record(&mut visitor);
        visitor.finish()
    }

    fn format_spans<SubscriberT>(
        &self,
        ctx: &FmtContext<'_, SubscriberT, DevLogFieldFormat>,
        writer: &mut Writer<'_>,
    ) -> fmt::Result
    where
        SubscriberT: Subscriber + for<'a> LookupSpan<'a>,
    {
        if let Some(scope) = ctx.event_scope() {
            let mut seen = false;

            for span in scope.from_root() {
                seen = true;

                let extensions = span.extensions();
                if let Some(fields) = &extensions.get::<FormattedFields<DevLogFieldFormat>>() {
                    if !fields.is_empty() {
                        write_field_name(writer, span.metadata().name())?;
                        write!(writer, "{fields}")?;
                    }
                }
            }

            if seen {
                writer.write_char(' ')?;
            }
        }

        Ok(())
    }

    fn format_source_location(
        &self,
        metadata: &Metadata<'static>,
        writer: &mut Writer<'_>,
    ) -> fmt::Result {
        let file_name = if self.display_filename {
            metadata.file()
        } else {
            None
        };
        let line_number = if self.display_line_number {
            metadata.line()
        } else {
            None
        };

        if file_name.is_none() && line_number.is_none() {
            return Ok(());
        }

        write_field_name(writer, "source")?;
        writer.write_char(' ')?;
        writer.set_color(COLOR_GRAY)?;

        match (file_name, line_number) {
            (Some(file_name), Some(line_number)) => {
                write!(writer, "{file_name}:{line_number}")?;
            }
            (Some(file_name), None) => {
                writer.write_str(file_name)?;
            }
            (None, Some(line_number)) => {
                write!(writer, "{line_number}")?;
            }
            (None, None) => {}
        }

        writer.reset_color()?;
        Ok(())
    }

    fn format_thread_info(&self, writer: &mut Writer<'_>) -> fmt::Result {
        let current_thread = thread::current();

        let thread_name = if self.display_thread_name {
            current_thread.name()
        } else {
            None
        };
        let thread_id = if self.display_thread_id {
            Some(current_thread.id())
        } else {
            None
        };

        if thread_name.is_none() && thread_id.is_none() {
            return Ok(());
        }

        write_field_name(writer, "thread")?;
        writer.write_char(' ')?;
        writer.set_color(COLOR_GRAY)?;

        match (thread_name, thread_id) {
            (Some(thread_name), Some(thread_id)) => {
                write!(writer, "{thread_name} [{thread_id:?}]")?;
            }
            (Some(thread_name), None) => {
                writer.write_str(thread_name)?;
            }
            (None, Some(_)) => {
                write!(writer, "{thread_id:?}")?;
            }
            (None, None) => {}
        }

        writer.reset_color()?;
        Ok(())
    }
}

fn write_field_name(writer: &mut Writer<'_>, field_name: &str) -> fmt::Result {
    writer.write_str("\n  ")?;
    writer.set_color(COLOR_CYAN)?;
    writer.write_str(field_name)?;
    writer.write_with_color(':', COLOR_GRAY)?;
    Ok(())
}

struct ThreadName<'a> {
    name: &'a str,
}

impl<'a> fmt::Display for ThreadName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::sync::atomic::{
            AtomicUsize,
            Ordering::{AcqRel, Acquire, Relaxed},
        };

        // Track the longest thread name length we've seen so far in an atomic,
        // so that it can be updated by any thread.
        static MAX_LEN: AtomicUsize = AtomicUsize::new(0);
        let len = self.name.len();
        // Snapshot the current max thread name length.
        let mut max_len = MAX_LEN.load(Relaxed);

        while len > max_len {
            // Try to set a new max length, if it is still the value we took a
            // snapshot of.
            match MAX_LEN.compare_exchange(max_len, len, AcqRel, Acquire) {
                // We successfully set the new max value
                Ok(_) => break,
                // Another thread set a new max value since we last observed
                // it! It's possible that the new length is actually longer than
                // ours, so we'll loop again and check whether our length is
                // still the longest. If not, we'll just use the newer value.
                Err(actual) => max_len = actual,
            }
        }

        // pad thread name using `max_len`
        write!(f, "{:>width$}", self.name, width = max_len)
    }
}
