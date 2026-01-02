# 🚀 Redis-RS: A High-Performance Redis Clone in Rust

[![progress-banner](https://backend.codecrafters.io/progress/redis/de784fac-4f12-4e3a-ac44-80f0489ddb68)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

A blazingly fast, thread-safe Redis implementation built from scratch in Rust. This project implements the Redis Serialization Protocol (RESP) and supports core Redis commands with an in-memory data store.

> **Built for the [CodeCrafters "Build Your Own Redis" Challenge](https://codecrafters.io/challenges/redis)** - A deep dive into network protocols, concurrent programming, and systems design.

## ✨ Features

### 🔧 Supported Commands

- **Key-Value Operations**
  - `GET key` - Retrieve values with automatic expiry handling
  - `SET key value [EX seconds] [PX milliseconds]` - Store values with optional TTL
  
- **List Operations**
  - `RPUSH key value [value ...]` - Append one or multiple values to a list
  - `LPUSH key value [value ...]` - Prepend one or multiple values to a list
  - `LRANGE key start stop` - Get a range of elements from a list
  - `LPOP key [count]` - Remove or or more elements from a list

- **Server Commands**
  - `PING [message]` - Test connectivity and server responsiveness
  - `ECHO message` - Echo back messages

### 🎯 Core Capabilities

- ⚡ **Async I/O** - Built on Tokio for high-performance concurrent connections
- 🔒 **Thread-Safe** - Uses `RwLock` for safe concurrent data access
- ⏰ **TTL Support** - Automatic key expiration with millisecond/second precision
- 🔄 **RESP Protocol** - Full implementation of Redis Serialization Protocol
- 💾 **In-Memory Storage** - Fast HashMap-based data storage
- 🧵 **Multi-threaded** - Handles multiple concurrent client connections
- 🛡️ **Type Safety** - Prevents operations on keys holding wrong data types

## 🏗️ Architecture

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │ RESP Protocol
┌──────▼──────────────────────┐
│   TCP Server (127.0.0.1:6379)│
└──────┬──────────────────────┘
       │
┌──────▼──────────────────────┐
│  Command Parser & Validator │
└──────┬──────────────────────┘
       │
┌──────▼──────────────────────┐
│    Command Execution        │
│  ┌────────────────────┐    │
│  │ Ping │ Echo │ SET  │    │
│  │ GET  │ RPUSH│LPUSH │    │
│  │ LRANGE│ ...  │     │    │
│  └────────────────────┘    │
└──────┬──────────────────────┘
       │
┌──────▼──────────────────────┐
│  In-Memory Database (MemDB) │
│  RwLock<HashMap<String, T>> │
└─────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- **Rust** `1.91+` (2024 edition)
- **Cargo** (comes with Rust)

### Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd rudis

# Build the project
cargo build --release

# Run the server
./your_program.sh
# or
cargo run
```

The Redis server will start on `127.0.0.1:6379` 🎉

### Usage Examples

Connect with `redis-cli` or any Redis client:

```bash
# Install redis-cli if needed
brew install redis  # macOS
# or use your system's package manager

# Connect to the server
redis-cli -h 127.0.0.1 -p 6379
```

```redis
# Test connectivity
127.0.0.1:6379> PING
PONG

127.0.0.1:6379> PING "Hello Redis!"
"Hello Redis!"

# Key-value operations
127.0.0.1:6379> SET mykey "Hello World"
OK

127.0.0.1:6379> GET mykey
"Hello World"

# Set with expiration (5 seconds)
127.0.0.1:6379> SET temp "expires soon" EX 5
OK

127.0.0.1:6379> GET temp
"expires soon"
# Wait 5 seconds...
127.0.0.1:6379> GET temp
(nil)

# List operations
127.0.0.1:6379> RPUSH mylist "first" "second" "third"
(integer) 3

127.0.0.1:6379> LPUSH mylist "zero"
(integer) 4

127.0.0.1:6379> LRANGE mylist 0 -1
1) "zero"
2) "first"
3) "second"
4) "third"
```

## 🧪 Testing

```bash
# Run tests (if implemented)
cargo test

# Run with verbose output
cargo run --verbose

# Check for compilation errors
cargo check
```

## 🔍 Implementation Details

### Thread Safety

The database uses `Arc<RwLock<MemDB>>` to ensure safe concurrent access:
- Multiple readers can access data simultaneously
- Writers get exclusive access
- Prevents data races and ensures consistency

### Data Expiration

Keys can expire using `EX` (seconds) or `PX` (milliseconds):
```rust
pub struct Data {
    value: Value,
    expires_at: Option<Instant>,
}
```

Expired keys are automatically detected on `GET` operations.

### RESP Protocol

Implements Redis Serialization Protocol with support for:
- Simple Strings (`+OK\r\n`)
- Errors (`-ERR ...\r\n`)
- Integers (`:42\r\n`)
- Bulk Strings (`$5\r\nhello\r\n`)
- Arrays (`*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n`)
- Null values (`$-1\r\n`, `*-1\r\n`, `_\r\n`)

### Type System

Supports multiple value types:
```rust
pub enum Value {
    String(Vec<u8>),
    List(Vec<String>),
    // Extensible for future types
}
```

Type checking prevents operations on incompatible types (e.g., `RPUSH` on a string value).

## 🛠️ Dependencies

```toml
anyhow = "1.0.59"      # Error handling with context
bytes = "1.3.0"        # Efficient byte buffer management
thiserror = "1.0.32"   # Custom error types
tokio = "1.23.0"       # Async runtime (full features)
```

## 🎓 Learning Resources

This project teaches:
- **Network Programming** - TCP server implementation
- **Protocol Design** - RESP protocol parsing & encoding
- **Concurrency** - Thread-safe data structures with Rust
- **Memory Management** - Efficient use of HashMap and lifetimes
- **Error Handling** - Using `Result`, `anyhow`, and `thiserror`
- **System Design** - Building scalable server applications

## 🚧 Future Enhancements

- [ ] Replication (master-slave)
- [ ] Persistence (RDB snapshots, AOF logs)
- [ ] Additional Redis commands (DEL, EXISTS, INCR, DECR)
- [ ] Pub/Sub support
- [ ] Transactions (MULTI/EXEC)
- [ ] Benchmarking suite
- [ ] Configuration file support
- [ ] Memory eviction policies

## 📝 License

This project is part of the CodeCrafters challenge but has been improved on top. Feel free to use it for educational purposes!

## 🙏 Acknowledgments

- [CodeCrafters](https://codecrafters.io) for the excellent challenge
- [Redis](https://redis.io) for the original implementation and protocol spec
- The Rust community for amazing tools and libraries

## 🤝 Contributing

This is a learning project, but suggestions and improvements are welcome! Feel free to:
- Open issues for bugs or questions
- Submit PRs for enhancements
- Share your own implementations

---

**Built with ❤️ and Rust** 🦀
