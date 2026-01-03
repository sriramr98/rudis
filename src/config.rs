use std::{
    fmt::Display,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

#[derive(Clone, PartialEq)]
pub enum Role {
    Master,
    Replica,
}

#[derive(Clone)]
pub struct Config {
    pub server: ServerInfo,
    pub replication: ReplicationInfo,
    pub stats: StatsInfo,
}

#[derive(Clone)]
pub struct ServerInfo {
    pub redis_version: String,
    pub os: String,
    pub arch_bits: String,
    pub tcp_port: u32,
}
impl ServerInfo {
    fn to_string(&self) -> String {
        let mut builder = String::new();
        builder.push_str("# Server\r\n");
        builder.push_str(&format!("redis_version: {}\r\n", self.redis_version));
        builder.push_str(&format!("os: {}\r\n", self.os));
        builder.push_str(&format!("arch_bits: {}\r\n", self.arch_bits));
        builder.push_str(&format!("tcp_port: {}\r\n", self.tcp_port));
        builder
    }
}

#[derive(Clone)]
pub struct ReplicationInfo {
    pub role: Role,
    pub master_host: Option<String>,
    pub master_port: Option<u32>,
    pub connected_slaves: Arc<AtomicUsize>,
    pub master_replid: String,
    pub master_repl_offset: u64,
}
impl ReplicationInfo {
    fn to_string(&self) -> String {
        let mut builder = String::new();
        builder.push_str("# Replication\r\n");
        builder.push_str(&format!(
            "role:{}\r\n",
            match self.role {
                Role::Master => "master",
                Role::Replica => "slave",
            }
        ));

        if self.role == Role::Master {
            builder.push_str(&format!(
                "connected_slaves:{}\r\n",
                self.connected_slaves.load(Ordering::Relaxed)
            ));
        }

        builder.push_str(&format!("master_replid:{}\r\n", self.master_replid));
        builder.push_str(&format!(
            "master_repl_offset:{}\r\n",
            self.master_repl_offset
        ));

        if let (Some(host), Some(port)) = (&self.master_host, self.master_port) {
            builder.push_str(&format!("master_host:{}\r\n", host));
            builder.push_str(&format!("master_port:{}\r\n", port));
        }

        builder
    }
}

#[derive(Clone)]
pub struct StatsInfo {
    pub total_connections_received: Arc<AtomicUsize>,
    pub total_commands_processed: Arc<AtomicUsize>,
}
impl StatsInfo {
    fn to_string(&self) -> String {
        format!(
            "total_connections_received:{}\r\n",
            self.total_connections_received.load(Ordering::Relaxed)
        ) + &format!(
            "total_commands_processed:{}\r\n",
            self.total_commands_processed.load(Ordering::Relaxed)
        )
    }
}

impl Config {
    pub fn new(
        role: Role,
        port: u32,
        master_host: Option<String>,
        master_port: Option<u32>,
    ) -> Self {
        let connected_slaves = Arc::new(AtomicUsize::new(0));
        Config {
            server: ServerInfo {
                redis_version: "0.1.0".to_string(),
                os: std::env::consts::OS.to_string(),
                arch_bits: if cfg!(target_pointer_width = "64") {
                    "64".to_string()
                } else {
                    "32".to_string()
                },
                tcp_port: port,
            },
            replication: ReplicationInfo {
                role,
                master_host,
                master_port,
                connected_slaves,
                master_replid: generate_random_alphanumeric(40),
                master_repl_offset: 0,
            },
            stats: StatsInfo {
                total_connections_received: Arc::new(AtomicUsize::new(0)),
                total_commands_processed: Arc::new(AtomicUsize::new(0)),
            },
        }
    }

    pub fn increment_connections(&self) {
        self.replication
            .connected_slaves
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.stats
            .total_connections_received
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn decrement_connections(&self) {
        self.replication
            .connected_slaves
            .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }

    //to_string returns a String representation of the entire config according to the RESP3 spec of INFO command (https://redis.io/docs/latest/commands/info/)
    pub fn to_string(&self) -> String {
        let mut builder = String::new();
        builder.push_str(&self.server.to_string());
        builder.push_str(&self.replication.to_string());
        builder.push_str(&self.stats.to_string());

        builder
    }
}

/// Generates a random alphanumeric string of the specified length
fn generate_random_alphanumeric(length: usize) -> String {
    use rand::Rng;

    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
