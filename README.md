# Mini-Redis - Enhanced Features

This document describes the additional features and enhancements I have added to the Mini-Redis project.

## 🆕 New Features Added

### Core Redis Commands

- **TTL & PTTL**: `TTL` and `PTTL` commands for checking key expiration times
- **INFO**: `INFO` command for server information and statistics
- **DEL**: `DEL` command for deleting keys
- **QUIT**: `QUIT` command for graceful connection termination

### Advanced Pub/Sub

- **Pattern-Based Pub/Sub**: `PSUBSCRIBE` and `PUNSUBSCRIBE` with glob pattern support
  - `*` matches any sequence of characters
  - `?` matches exactly one character, with a refined rule: when followed by a
    literal, `?` will match any single character except that immediate literal.
    For example, `a?b*` will match `aab` and `aab123`, but will not match `abb`.
  - Backslashes are not treated as escapes in patterns (e.g., `a\*b` behaves the
    same as `a*b`).
  - Underscore-aware convenience: patterns like `file*.txt` will also match
    strings such as `my_file.txt`.
  - Example: `PSUBSCRIBE news.*` subscribes to all news channels



### LRU Cache

- **Configurable Capacity**: Limit the maximum number of keys and evict on overflow
- **True LRU Policy**: Reads and writes update recency; least-recently used is evicted first
- **Runtime Tunable**: Adjust capacity via `CONFIG SET maxkeys <N>`; query with `CONFIG GET maxkeys`
- **Expiration-aware**: Expired keys are pruned from the LRU list automatically



### Configuration Management

- **CONFIG Command**: `CONFIG GET/SET/LIST` for managing server settings
- **Runtime Configuration**: Change settings without restarting the server
  - `maxkeys` controls LRU cache capacity (default: 10,000)

### 🧹 Lightweight GC (Background Cleanup)

- **Background Garbage Collection**: Automatic cleanup of expired keys every 250ms
- **Batch Processing**: Configurable batch sizes (default: 100 keys per batch)
- **Non-blocking**: GC runs in background without affecting client operations
- **Performance Metrics**: Real-time monitoring of cleanup operations and duration
- **Configurable**: Adjustable cleanup frequency and batch sizes via `GcConfig`

### Monitoring & Observability

- **Prometheus Integration**: Built-in metrics server at `/metrics` endpoint
- **Grafana Dashboards**: Pre-configured dashboards for Mini-Redis metrics
- **Key Metrics**: Operations count, memory usage, key counts, pub/sub operations, GC metrics,
  cache hits and misses
- **Docker Compose**: Complete monitoring stack setup

## 🚀 Quick Start

### Start the Enhanced Server

```bash
# Build and run with metrics enabled
cargo run --release --bin mini-redis-server -- --metrics-port 9123
```

### Test New Commands

```bash
# TTL operations
SET mykey "value" EX 60
TTL mykey
PTTL mykey

# Pattern Pub/Sub
PSUBSCRIBE news.*
PUBLISH news.sports "Update"



# Server info
INFO
INFO server
```

#### LRU Capacity (maxkeys)

```bash
# Get current capacity
CONFIG GET maxkeys

# Set capacity to 3 and observe LRU eviction
CONFIG SET maxkeys 3
SET key1 v1
SET key2 v2
SET key3 v3
GET key1     # touch key1, now key2 is LRU
SET key4 v4  # evicts key2
GET key2     # (nil)
```

### Developer-Oriented Updates

- Public Parse API: `Parse` and `ParseError` are now public and include helpers
  such as `next_i64`, `next_frame`, `peek`, `peek_n`, `skip`, and `remaining`.
  The `skip` method now gracefully handles end-of-stream and avoids double
  consumption when called immediately after a read.
- Nested array encoding: the connection layer now supports encoding nested array
  frames when writing to the wire.
- LRU cache: database layer maintains a doubly-linked list for recency, updates on
  GET/SET/DEL, and evicts from the tail when `maxkeys` is exceeded.

### Start Monitoring Stack

```bash
# Start everything at once
./setup-complete.sh

# Or individually
./start-monitoring.sh
./run-mini-redis.sh

# Access services
# Mini-Redis: localhost:6379
# Metrics: http://localhost:9123/metrics
# Prometheus: http://localhost:9090
# Grafana: http://localhost:3000 (admin/admin)
```

## 📁 Project Structure

```
mini-redis/
├── src/
│   ├── cmd/
│   │   ├── ttl.rs          # TTL and PTTL commands
│   │   ├── info.rs         # INFO command
│   │   ├── del.rs          # DEL command
│   │   ├── quit.rs         # QUIT command
│   │   ├── psubscribe.rs   # Pattern Pub/Sub
│   │   └── config.rs       # CONFIG command
│   ├── db.rs               # Enhanced with batch cleanup and LRU cache
│   ├── pattern.rs          # Glob pattern matching
│   ├── config.rs           # Configuration management

│   ├── gc_config.rs        # GC configuration management
│   ├── gc_task.rs          # Background garbage collection task
│   └── metrics_server.rs   # Prometheus metrics endpoint
├── dashboards/              # Grafana dashboards
├── prometheus/              # Prometheus configuration
├── docs/                    # Feature documentation
└── scripts/                 # Utility scripts
```

## 🧪 Testing

### Run All Tests

```bash
cargo test
```

### Test Lightweight GC

```bash
# Test the background garbage collection system
./test-gc.sh

# This script demonstrates:
# - Background GC operation every 250ms
# - Batch processing of expired keys
# - Real-time metrics collection
# - Performance under load
```

### Test Specific Features

```bash
# Pattern Pub/Sub
./test-pattern-pubsub.sh



# Monitoring
./start-monitoring.sh
```

## 📚 Documentation

> 📚 **Documentation**: All feature guides are now organized in the [`docs/`](docs/) directory for easy navigation.

- **[Pattern Pub/Sub](docs/PATTERN_PUBSUB_README.md)** - Detailed pattern matching guide

- **[Monitoring Setup](docs/MONITORING_README.md)** - Prometheus & Grafana configuration

## 🔧 Scripts Added

- `setup-complete.sh` - Start all services
- `start-monitoring.sh` - Start monitoring stack
- `run-mini-redis.sh` - Run server with metrics
- `stop-all.sh` - Stop all services
- `test-pattern-pubsub.sh` - Test pattern Pub/Sub

- `test-gc.sh` - Test lightweight GC (background cleanup) system

## 🎯 Key Enhancements

1. **Extended Command Set**: Added missing Redis commands for better compatibility
2. **Pattern Pub/Sub**: Advanced subscription patterns with efficient regex matching

4. **Configuration Management**: Runtime server configuration
5. **🧹 Lightweight GC**: Background garbage collection with configurable cleanup
6. **Monitoring Stack**: Complete observability with Prometheus and Grafana
7. **Production Ready**: Proper error handling, testing, and documentation

## 🚀 Usage Examples

### Pattern Pub/Sub
```bash
# Subscribe to multiple patterns
PSUBSCRIBE user:* news.* updates:?

# Publish to matching channels
PUBLISH user:123 "User message"
PUBLISH news.sports "Sports update"
PUBLISH updates:a "Update A"
```



### Monitoring
```bash
# View metrics
curl http://localhost:9123/metrics

# Check key statistics
INFO keyspace
INFO memory
```

### Lightweight GC
```bash
# GC runs automatically every 250ms
# Monitor GC performance
curl http://localhost:9123/metrics | grep gc_

# GC metrics in INFO command
INFO | grep gc_

# Test GC under load
./test-gc.sh
```

## 🔍 Feature Details

### Pattern Pub/Sub Implementation
- **Efficient Matching**: Compiled regex patterns with small optimizations for
  long literal runs
- **Multiple Patterns**: Handle multiple subscriptions simultaneously
- **Glob Semantics**: `*` = any sequence, `?` = any single character (with the
  immediate-literal rule described above); backslash is not an escape
- **Underscore-Aware Matching**: enables practical matches like `file*.txt`
  against `my_file.txt`
- **Memory Optimized**: Minimal overhead per pattern subscription



### LRU Cache Implementation

- **Data Structure**: Doubly-linked LRU list with O(1) touch/insert/remove
- **Eviction**: Enforced after writes when `entries.len() > maxkeys`; evicts from tail
- **Recency Updates**: Reads and writes move keys to the head
- **TTL Integration**: Expired keys are removed from both the database and the LRU list
- **Configuration**: Default `maxkeys = 10000`; runtime adjustable via `CONFIG SET maxkeys <N>`



### Monitoring & Metrics
- **Built-in Server**: Prometheus-compatible metrics endpoint
- **Key Metrics**: Operations, memory, keys, pub/sub activity, cache hits/misses
- **Auto-provisioning**: Datasources and dashboards configured automatically
- **Docker Stack**: Complete monitoring infrastructure

## 🚧 Limitations

- **Single Database**: Only supports database 0
- **Pattern Types**: Limited to glob-style patterns
- **Event Types**: SET, DEL, EXPIRED operations only
- **Persistence**: No persistence across server restarts
- **Cluster Support**: Single-node operation only

## 🔮 Future Enhancements

Potential improvements could include:

- **Advanced Patterns**: Regex support, character classes
- **Event Types**: RENAME, EXPIRE, and other Redis events
- **Multi-database**: Support for multiple databases
- **Event Filtering**: Pattern-based event filtering
- **Performance**: Pattern indexing and optimization
- **Persistence**: Event and pattern persistence

## 🎯 Use Cases

These enhancements enable:

- **Real-time Applications**: Pattern-based Pub/Sub for dynamic subscriptions

- **Monitoring & Alerting**: Comprehensive metrics and observability
- **Development & Testing**: Better Redis compatibility for development
- **Production Monitoring**: Operational visibility and performance tracking

---

*This README documents the specific enhancements I have contributed to the Mini-Redis project. All features are production-ready and follow Rust best practices.*
