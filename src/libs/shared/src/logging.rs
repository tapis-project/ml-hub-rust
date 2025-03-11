use log::{debug, info, warn, error, trace};

#[derive(Debug, Copy, Clone)]
pub struct SharedLogger {}

impl SharedLogger {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn debug(&self, msg: &str) {
        debug!("{}", msg)
    }

    pub fn info(&self, msg: &str) {
        info!("{}", msg)
    }

    pub fn warn(&self, msg: &str) {
        warn!("{}", msg)
    }

    pub fn error(&self, msg: &str) {
        error!("{}", msg)
    }

    pub fn trace(&self, msg: &str) {
        trace!("{}", msg)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GlobalLogger {}

impl GlobalLogger {
    pub fn debug(msg: &str) {
        debug!("{}", msg)
    }

    pub fn info(msg: &str) {
        info!("{}", msg)
    }

    pub fn warn(msg: &str) {
        warn!("{}", msg)
    }

    pub fn error(msg: &str) {
        error!("{}", msg)
    }

    pub fn trace(msg: &str) {
        trace!("{}", msg)
    }
}