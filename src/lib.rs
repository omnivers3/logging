extern crate log;
extern crate sink;

#[macro_use]
mod macros;

pub use log::{ logger, Level, Record };
use sink::Sink;

#[derive(Clone, Debug)]
/// Captures the trace level details filled in by the logger implementation
pub struct Data {
    target: String,
    value: String,
    module_path: Option<String>,
    file: Option<String>,
    line: Option<u32>,
}

impl Data {
    pub fn full(target: String, value: String, module_path: String, file: String, line: u32) -> Self {
        Data {
            target,
            value,
            module_path: Some(module_path),
            file: Some(file),
            line: Some(line),
        }
    }
}

#[derive(Clone, Debug)]
/// Wraps the trace data structure in a union mapping to logging levels
pub enum LoggingEvents {
    Debug(Data),
    Error(Data),
    Info(Data),
    Trace(Data),
    Warning(Data),
}

impl LoggingEvents {
    /// Decomposes the trace data into a formal logging implementation emit event
    /// 
    /// Calls logger().log(...) constructing the builder inline
    /// 
    /// ** Has side-effects
    pub fn log<'a>(&'a self) {
        let (level, data) = match self {
            LoggingEvents::Debug(data) => (Level::Debug, data),
            LoggingEvents::Info(data) => (Level::Info, data),
            LoggingEvents::Error(data) => (Level::Error, data),
            LoggingEvents::Trace(data) => (Level::Trace, data),
            LoggingEvents::Warning(data) => (Level::Warn, data),
        };
        logger().log(&Record::builder()
            .level(level)
            .target(&data.target)
            .module_path(data.module_path.as_ref().map(|x| &**x))
            .file(data.file.as_ref().map(|x| &**x))
            .line(data.line)
            .args(format_args!("{}", data.value))
            .build()
        );
    }
}

/// Trait alias for null result sink accepting LoggingEvents
pub trait LoggingSink: Sink<TInput = LoggingEvents, TResult = ()> {}

/// Extends the Sink trait to all qualifying LoggingSinks
impl<T> LoggingSink for T where T: Sink<TInput = LoggingEvents, TResult = ()> {}

#[derive(Clone)]
/// A Sink implementation which maps the LoggingEvents enum from this crate into pluggable
/// logging frameworks enabling composition into a Sink based simulation system
/// ** Has side effects
pub struct Logging {}

impl<'a> Logging {
    /// Builds an empty Logging struct, configuring this sink is done indirectly via the
    /// methods described in the crate: https://crates.io/crates/log
    pub fn new() -> Self {
        Logging {}
    }
}

/// Extend the Sink trait for the Logging struct which triggers an emit of the passed event
impl Sink for Logging {
    type TInput = LoggingEvents;
    type TResult = ();

    fn send(&self, input: LoggingEvents) -> () {
        input.log();
    }
}