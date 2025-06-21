use log::{log, Level};

#[derive(Debug, Copy, Clone)]
pub struct SharedLogger {}

impl SharedLogger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn debug(&self, msg: &str) {
        log!(Level::Debug, "{}", msg);
    }

    pub fn info(&self, msg: &str) {
        log!(Level::Info, "{}", msg);
    }

    pub fn warn(&self, msg: &str) {
        log!(Level::Warn, "{}", msg);
    }

    pub fn error(&self, msg: &str) {
        log!(Level::Error, "{}", msg);
    }

    pub fn trace(&self, msg: &str) {
        log!(Level::Trace, "{}", msg);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GlobalLogger {}

impl GlobalLogger {
    pub fn debug(msg: &str) {
        log!(Level::Debug, "{}", msg);
    }

    pub fn info(msg: &str) {
        log!(Level::Info, "{}", msg);
    }

    pub fn warn(msg: &str) {
        log!(Level::Warn, "{}", msg);
    }

    pub fn error(msg: &str) {
        log!(Level::Error, "{}", msg);
    }

    pub fn trace(msg: &str) {
        log!(Level::Trace, "{}", msg);
    }
}