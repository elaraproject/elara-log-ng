// Much thanks to https://github.com/takeshixx/python-tinylogs
// of which elara-log takes a lot of its design from

#![allow(non_camel_case_types)]
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;
use chrono;
use chrono::{Timelike, Datelike};

// mod time;

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
    FileHandler(PathBuf)
}

impl LogfileType {
    fn get_path(&mut self) -> Option<PathBuf> {
        match self {
            LogfileType::FileHandler(p) => Some(p.to_path_buf()),
            _ => None
        }
    }
}

pub struct Logger {
    debug: bool,
    file: LogfileType,
    multi: bool
}

impl Logger {
    fn _timestamp(&self) -> String {
        // let now = SystemTime::now();
        // let time = time::fmt("%Y-%m-%dT%H:%M:%S");
        let now = chrono::offset::Local::now();
        format!("{}-{}-{} {}:{}:{}",
          now.year(),
          now.month(),
          now.day(),
          now.hour(),
          now.minute(),
          now.second())
    }

    fn _is_logfile(&mut self) -> bool {
        match &self.file {
            LogfileType::FileHandler(_) => true,
            _ => false
        }
    }

    fn _is_stdout(&mut self) -> bool {
        match &self.file {
            LogfileType::FileHandler(_) => {
                if self.multi {
                    true
                } else {
                    false
                }
            },
            LogfileType::Nofile => true
        }
    }

    pub fn new() -> Logger {
        Logger {
            debug: false,
            file: LogfileType::Nofile,
            multi: false
        }
    }

    pub fn set_debug(&mut self) {
        self.debug = true;
    }

    pub fn set_multi(&mut self) {
        self.multi = true;
    }

    pub fn set_logfile(&mut self, path_str: &str) {
        let path = Path::new(&path_str);
        self.file = LogfileType::FileHandler(path.to_path_buf());
    }

    fn print(&mut self, title: &str, color: &str, msg: &str) {
        if self._is_logfile() {
            let path = self.file.get_path().unwrap();
            let mut logfile = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path).unwrap();
            let log_message = format!("[{}] {} {}\n",
                self._timestamp(),
                title,
                msg);
            write!(&mut logfile, "{}", log_message).unwrap();
        }

        if self._is_stdout() {
            println!("{} {}{}{} {}",
                self._timestamp(),
                DELIMITER,
                format!("{}{}{}{}{}", ESCAPE, color, title, ESCAPE, DELIMITER),
                ESCAPE,
                msg
            );
        }
    }
    pub fn info(&mut self, msg: &str) {
        self.print(INFO_TEXT, INFO_COLOR, msg);
    }
    pub fn success(&mut self, msg: &str) {
        self.print(SUCCESS_TEXT, SUCCESS_COLOR, msg);
    }
    pub fn warn(&mut self, msg: &str) {
        self.print(WARNING_TEXT, WARNING_COLOR, msg);
    }
    pub fn debug(&mut self, msg: &str) {
        self.print(DEBUG_TEXT, DEBUG_COLOR, msg);
    }
    pub fn error(&mut self, msg: &str) {
        self.print(ERROR_TEXT, ERROR_COLOR, msg);
        exit(1);
    }
}
