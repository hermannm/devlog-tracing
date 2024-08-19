use std::error::Error;

use tracing_subscriber::fmt::{time::FormatTime, SubscriberBuilder};

use crate::{
    event_format::DevLogEventFormat, field_format::DevLogFieldFormat, time_format::DevLogTimeFormat,
};

pub struct DevLogSubscriberBuilder<TimeFormatT> {
    field_format: DevLogFieldFormat,
    event_format: DevLogEventFormat<TimeFormatT>,
}

impl Default for DevLogSubscriberBuilder<DevLogTimeFormat> {
    fn default() -> Self {
        Self {
            field_format: DevLogFieldFormat::default(),
            event_format: DevLogEventFormat::default(),
        }
    }
}

impl<TimeFormatT> DevLogSubscriberBuilder<TimeFormatT> {
    /// Uses the given [`FormatTime`] implementation for log time formatting.
    pub fn with_timer<NewTimeFormatT: FormatTime>(
        self,
        timer: NewTimeFormatT,
    ) -> DevLogSubscriberBuilder<NewTimeFormatT> {
        DevLogSubscriberBuilder {
            field_format: self.field_format,
            event_format: DevLogEventFormat {
                timer,
                // We have to set every field here for the generics to work
                display_timestamp: self.event_format.display_timestamp,
                display_target: self.event_format.display_target,
                display_level: self.event_format.display_level,
                display_thread_id: self.event_format.display_thread_id,
                display_thread_name: self.event_format.display_thread_name,
                display_filename: self.event_format.display_filename,
                display_line_number: self.event_format.display_line_number,
            },
        }
    }

    /// Excludes timestamps from log events.
    pub fn without_time(self) -> DevLogSubscriberBuilder<()> {
        DevLogSubscriberBuilder {
            field_format: self.field_format,
            event_format: DevLogEventFormat {
                timer: (),
                display_timestamp: false,
                // We have to set every field here for the generics to work
                display_target: self.event_format.display_target,
                display_level: self.event_format.display_level,
                display_thread_id: self.event_format.display_thread_id,
                display_thread_name: self.event_format.display_thread_name,
                display_filename: self.event_format.display_filename,
                display_line_number: self.event_format.display_line_number,
            },
        }
    }

    /// Whether to show the target of a log event (where it originated).
    pub fn with_target(mut self, display_target: bool) -> Self {
        self.event_format.display_target = display_target;
        self
    }

    /// Whether to show the level of a log event (INFO, WARN, ERROR etc.).
    pub fn with_level(mut self, display_level: bool) -> Self {
        self.event_format.display_level = display_level;
        self
    }

    /// Whether to show the ID of the current thread in log events.
    pub fn with_thread_ids(mut self, display_thread_id: bool) -> Self {
        self.event_format.display_thread_id = display_thread_id;
        self
    }

    /// Whether to show the name of the current thread in log events.
    pub fn with_thread_names(mut self, display_thread_name: bool) -> Self {
        self.event_format.display_thread_name = display_thread_name;
        self
    }

    /// Whether to show the source code file path where a log event was logged.
    pub fn with_file(mut self, display_filename: bool) -> Self {
        self.event_format.display_filename = display_filename;
        self
    }

    /// Whether to show the line number in a source code file path where a log event was logged.
    pub fn with_line_number(mut self, display_line_number: bool) -> Self {
        self.event_format.display_line_number = display_line_number;
        self
    }

    /// Whether to show the source code location (file path + line number) where a log event was
    /// logged. Equivalent to calling [`DevLogSubscriberBuilder::with_file`] and
    /// [`DevLogSubscriberBuilder::with_line_number`] with the same value.
    pub fn with_source_location(self, display_location: bool) -> Self {
        self.with_line_number(display_location)
            .with_file(display_location)
    }
}

impl<TimeFormatT> DevLogSubscriberBuilder<TimeFormatT>
where
    TimeFormatT: FormatTime + Send + Sync + 'static,
{
    pub fn finish(self) -> impl tracing::Subscriber {
        self.build_fmt_subscriber().finish()
    }

    pub fn try_init(self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.build_fmt_subscriber().try_init()
    }

    pub fn init(self) {
        self.build_fmt_subscriber().init()
    }

    fn build_fmt_subscriber(
        self,
    ) -> SubscriberBuilder<DevLogFieldFormat, DevLogEventFormat<TimeFormatT>> {
        tracing_subscriber::fmt()
            .fmt_fields(self.field_format)
            .event_format(self.event_format)
    }
}
