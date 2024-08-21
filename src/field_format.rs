use core::fmt;

use super::color::{COLOR_CYAN, COLOR_GRAY, COLOR_RESET};
use tracing::field::{Field, Visit};
use tracing_subscriber::{
    field::{MakeVisitor, VisitFmt, VisitOutput},
    fmt::format::Writer,
};

use std::{error::Error, fmt::Debug};

/// A log field formatter for `tracing`, with a prettified, newline-delimited format. This
/// aims to improve readability over the default log field format, which appends log fields on the
/// same line, making it hard to read when multiple fields are appended.
///
/// ### Usage
///
/// ```rust
/// tracing_subscriber::fmt()
///     .fmt_fields(DevLogFieldFormatter::new())
///     .init();
/// ```
///
/// ### Example
///
/// This example log:
/// ```rust
/// error!(reason = "Bad things", severity = "BAD", "Something went wrong");
/// ```
/// ...gets printed like this:
/// ```text
/// ERROR Something went wrong
///   reason: "Bad things"
///   severity: "BAD"
/// ```
/// If your terminal supports ASCII color codes, the log field names ("reason" and "severity") above
/// will be colored, to distinguish them from field values.
#[derive(Default)]
pub(crate) struct DevLogFieldFormat {
    // Prevents direct struct initialization, so we can add fields here later as a non-breaking
    // change.
    _private: (),
}

impl<'a> MakeVisitor<Writer<'a>> for DevLogFieldFormat {
    type Visitor = DevLogFieldVisitor<'a>;

    fn make_visitor(&self, writer: Writer<'a>) -> Self::Visitor {
        DevLogFieldVisitor {
            writer,
            result: Ok(()),
            first_visit: true,
            delimiter: "\n        ", // 8-space indent
        }
    }
}

impl DevLogFieldFormat {
    pub(crate) fn make_field_visitor<'a>(&self, writer: Writer<'a>) -> DevLogFieldVisitor<'a> {
        DevLogFieldVisitor {
            writer,
            result: Ok(()),
            first_visit: true,
            delimiter: "\n  ", // 2-space indent
        }
    }
}

pub(crate) struct DevLogFieldVisitor<'a> {
    writer: Writer<'a>,
    result: fmt::Result,
    first_visit: bool,
    delimiter: &'static str,
}

impl<'a> DevLogFieldVisitor<'a> {
    fn write_field(&mut self, field: &Field, value: &dyn Debug) {
        self.write_field_name(field);
        if self.result.is_err() {
            return;
        }
        self.result = write!(self.writer, " {value:?}");
    }

    fn write_string_field(&mut self, field: &Field, value: &str) {
        self.write_field_name(field);
        if self.result.is_err() {
            return;
        }
        self.result = write!(self.writer, " {value}")
    }

    fn write_field_name(&mut self, field: &Field) {
        self.result = if self.writer.has_ansi_escapes() {
            write!(self.writer, "{COLOR_CYAN}{field}{COLOR_GRAY}:{COLOR_RESET}")
        } else {
            write!(self.writer, "{field}:")
        };
    }

    fn write_string_list_item(&mut self, value: &str) {
        if self.result.is_err() {
            return;
        }

        self.result = if self.writer.has_ansi_escapes() {
            write!(
                self.writer,
                "{}  {COLOR_GRAY}-{COLOR_RESET} {value}",
                self.delimiter,
            )
        } else {
            write!(self.writer, "{}  - {value}", self.delimiter)
        }
    }

    fn delimit(&mut self) {
        if self.result.is_err() {
            return;
        }

        let delimiter = self.delimiter;
        self.result = self.writer().write_str(delimiter);
    }
}

impl<'a> Visit for DevLogFieldVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        // A log line may or may not contain a main log message, which will be the first field and
        // have the name "message". If we do get such a message, we don't want to delimit or write
        // field name for it.
        if self.first_visit && field.name() != "message" {
            self.first_visit = false;
        }

        if !self.first_visit {
            self.delimit();
        }

        if self.result.is_err() {
            return;
        }

        if self.first_visit {
            self.first_visit = false;
            self.result = write!(self.writer, "{value:?}")
        } else {
            self.write_field(field, value)
        }
    }

    fn record_error(&mut self, field: &Field, mut error: &(dyn Error + 'static)) {
        // If an error is the first message, that means we haven't got a main log message (since
        // that will be the first message, called "message"). In this case, we add special case
        // handling if the field is called "cause", using the error's message as the main log
        // message, and adding the error's cause as the "cause" log field, if any.
        if self.first_visit {
            self.first_visit = false;

            if field.name() == "cause" {
                self.result = self.writer().write_str(&error.to_string());

                match error.source() {
                    Some(cause) => error = cause,
                    None => return,
                }
            }
        }

        self.delimit();
        if self.result.is_err() {
            return;
        }

        // If the error has no cause, we just write the error string
        let Some(cause) = error.source() else {
            self.write_string_field(field, &error.to_string());
            return;
        };

        // If the error has a cause, we format it as a list where each cause is a list item
        self.write_field_name(field);
        self.write_string_list_item(&error.to_string());
        self.write_string_list_item(&cause.to_string());
        while let Some(cause) = cause.source() {
            self.write_string_list_item(&cause.to_string());
        }
    }
}

impl<'a> VisitOutput<fmt::Result> for DevLogFieldVisitor<'a> {
    fn finish(self) -> fmt::Result {
        self.result
    }
}

impl<'a> VisitFmt for DevLogFieldVisitor<'a> {
    fn writer(&mut self) -> &mut dyn fmt::Write {
        &mut self.writer
    }
}
