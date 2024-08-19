pub use subscriber_builder::DevLogSubscriberBuilder;
pub use time_format::DevLogTimeFormat;

mod color;
mod event_format;
mod field_format;
mod subscriber_builder;
mod time_format;

pub fn fmt() -> DevLogSubscriberBuilder<DevLogTimeFormat> {
    DevLogSubscriberBuilder::default()
}
