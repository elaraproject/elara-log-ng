/*!
This crate provides basic logging capabilities for the
[Project Elara](https://github.com/elaraproject/) suite of
open-source software libraries. It can also be used as
a general-purpose lightweight logging library. It has
just five logging macros:

```rust
debug!(some_debug_message)
error!(some_error_message)
info!(some_info_message)
success!(some_success_message)
warn!(some_success_message)
```

The macros accept the same format strings as `println!` which
allows using string substitutions:

```rust
info!("The Answer to {} is {}.", 
	  "the Ultimate Question of Life, the Universe, and Everything", 
	  "42");
```

To use it in your project, install it with cargo by running:

```sh
# this crate is listed under a different
# name to avoid a naming class with a previous
# deprecated version of the same library
cargo add elara-log-ng
```

Then import the crate and initialize the logger
in your `main()` **before** calling any of the logging macros:

```rust
// required in all cases
// (including if you're using
// elara-log in a library)
use elara_log::prelude::*;

fn main() {
	// for executables/applications only
	// add this before calling any of
	// the logging macros
	Logger::new().init().unwrap();
}
```
*/

// Much thanks to https://github.com/takeshixx/python-tinylogs
// and https://github.com/borntyping/rust-simple_logger
// and https://github.com/rust-lang/log
// of which elara-log takes a lot of its design from

#![allow(non_camel_case_types)]
use sys_time::DateTime;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

// mod time;
pub mod prelude {
    pub use crate::{debug, error, info, success, warn, Logger};
}

const INFO_COLOR: &str = "\x1b[0;34m";
const SUCCESS_COLOR: &str = "\x1b[0;32m";
const WARNING_COLOR: &str = "\x1b[0;33m";
const ERROR_COLOR: &str = "\x1b[0;31m";
const DEBUG_COLOR: &str = "\x1b[0;35m";
const ESCAPE: &str = "\x1b[0m";
const DELIMITER: &str = "\x1b[1;1m";
const INFO_TEXT: &str = "INFO";
const SUCCESS_TEXT: &str = "SUCCESS";
const WARNING_TEXT: &str = "WARNING";
const ERROR_TEXT: &str = "ERROR";
const DEBUG_TEXT: &str = "DEBUG";

enum LogfileType {
    Nofile,
    FileHandler(PathBuf),
}

impl LogfileType {
    fn get_path(&self) -> Option<PathBuf> {
        match self {
            LogfileType::FileHandler(p) => Some(p.to_path_buf()),
            _ => None,
        }
    }
}

pub struct Logger {
    debug: bool,
    file: LogfileType,
    multi: bool,
}

impl Logger {
    fn _timestamp(&self) -> String {
        // let now = SystemTime::now();
        // let time = time::fmt("%Y-%m-%dT%H:%M:%S");
        // note: UTC time not current time
        let now = DateTime::now_utc();
        format!(
            "{}-{}-{} {}:{}:{}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        )
    }

    fn _is_logfile(&self) -> bool {
        match &self.file {
            LogfileType::FileHandler(_) => true,
            _ => false,
        }
    }

    fn _is_stdout(&self) -> bool {
        match &self.file {
            LogfileType::FileHandler(_) => {
                if self.multi {
                    true
                } else {
                    false
                }
            }
            LogfileType::Nofile => true,
        }
    }

    #[must_use = "You must call init() afterwards to begin logging"]
    pub fn new() -> Logger {
        Logger {
            debug: false,
            file: LogfileType::Nofile,
            multi: false,
        }
    }

    pub fn init(self) -> Result<(), LoggerInitError> {
        set_logger(Box::new(self))?;
        Ok(())
    }

    pub fn set_debug(&mut self) {
        self.debug = true;
    }

    pub fn set_multi(&mut self) {
        self.multi = true;
    }

    pub fn set_logfile(&mut self, path_str: &str) {
        let path = Path::new(&path_str).to_path_buf();
        self.file = LogfileType::FileHandler(path);
    }

    pub fn get_path(self) -> Option<PathBuf> {
        self.file.get_path()
    }

    fn print(&self, title: &str, color: &str, msg: String) {
        if self._is_logfile() {
            let path = &self.file.get_path().unwrap();
            let mut logfile = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .unwrap();
            let log_message = format!("[{}] {} {}\n", self._timestamp(), title, msg);
            write!(&mut logfile, "{}", log_message).unwrap();
        }

        if self._is_stdout() {
            println!(
                "{} {}{}{} {}",
                self._timestamp(),
                DELIMITER,
                format!("{}{}{}{}{}", ESCAPE, color, title, ESCAPE, DELIMITER),
                ESCAPE,
                msg
            );
        }
    }
}

pub enum LogLevel {
    Info,
    Success,
    Warn,
    Debug,
    Error,
}

impl Log for Logger {
    fn log(&self, level: LogLevel, msg: String){
        match level {
            LogLevel::Info => self.print(INFO_TEXT, INFO_COLOR, msg),
            LogLevel::Success => self.print(SUCCESS_TEXT, SUCCESS_COLOR, msg),
            LogLevel::Warn => self.print(WARNING_TEXT, WARNING_COLOR, msg),
            LogLevel::Debug => self.print(DEBUG_TEXT, DEBUG_COLOR, msg),
            LogLevel::Error => self.print(ERROR_TEXT, ERROR_COLOR, msg)

        }
    }
}

pub trait Log {
    fn log(&self, level: LogLevel, msg: String);
}

#[derive(Debug)]
pub struct LoggerInitError;

// Dummy logger for non-initialized logger
struct NoLogger;

impl Log for NoLogger {
    fn log(&self, _level: LogLevel, _msg: String) {}
}

impl fmt::Display for LoggerInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Logger was initialized more than once or couldn't initialize.")
    }
}

static mut LOGGER: &dyn Log = &NoLogger;

// Uses set_boxed_logger implementation in the `log` crate
// as it is hard to guarantee 'static for the logger
pub fn set_logger(logger: Box<dyn Log>) -> Result<(), LoggerInitError> {
    _set_logger(|| Box::leak(logger))
}

fn _set_logger<F>(make_logger: F) -> Result<(), LoggerInitError>
where
    F: FnOnce() -> &'static dyn Log,
{
    unsafe {
        LOGGER = make_logger();
    }
    Ok(())
}

pub fn logger() -> &'static dyn Log {
    unsafe { LOGGER }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::logger().log($crate::LogLevel::Info, format!($($arg)+)))
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crate::logger().log($crate::LogLevel::Warn, format!($($arg)+)))
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ($crate::logger().log($crate::LogLevel::Debug, format!($($arg)+)))
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => ($crate::logger().log($crate::LogLevel::Success, format!($($arg)+)))
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ($crate::logger().log($crate::LogLevel::Error, format!($($arg)+)))
}
