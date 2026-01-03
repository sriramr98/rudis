use clap::Args;
use std::sync::{Arc, atomic::AtomicUsize};

#[derive(Clone)]
pub struct Config {
    role: String,
    connected_clients: Arc<AtomicUsize>,
}

impl Config {
    pub fn new(role: String) -> Self {
        Config {
            role,
            connected_clients: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn connected_clients(&self) -> usize {
        self.connected_clients
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn increment_connections(&self) {
        self.connected_clients
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn decrement_connections(&self) {
        self.connected_clients
            .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
}
