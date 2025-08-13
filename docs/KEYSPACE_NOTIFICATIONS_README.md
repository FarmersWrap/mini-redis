# Keyspace Notifications

This document describes the implementation of keyspace notifications in Mini-Redis, which allows clients to subscribe to database events like key creation, deletion, and expiration.

## 🎯 Overview

Keyspace notifications extend Mini-Redis by publishing events to special channels whenever certain database operations occur. This enables clients to build reactive applications that can respond to database changes in real-time.

## ✨ Features

- **SET Events**: Notifications when keys are created or updated
- **DEL Events**: Notifications when keys are deleted
- **EXPIRED Events**: Notifications when keys expire due to TTL
- **Configurable**: Enable/disable notifications via CONFIG command
- **Environment Variable Support**: Set default via `MINI_REDIS_NOTIFY_KEYSPACE_EVENTS`
- **Redis Protocol Compatible**: Follows standard Redis keyspace notification format

## 🚀 Usage Examples

### Basic Configuration

```bash
# Enable keyspace notifications
CONFIG SET notify-keyspace-events 1

# Disable keyspace notifications
CONFIG SET notify-keyspace-events 0

# Check current setting
CONFIG GET notify-keyspace-events

# List all configuration
CONFIG LIST
```

### Subscribing to Events

```bash
# Subscribe to SET events
SUBSCRIBE __keyevent@0__:set

# Subscribe to DEL events
SUBSCRIBE __keyevent@0__:del

# Subscribe to EXPIRED events
SUBSCRIBE __keyevent@0__:expired
```

### Testing Event Generation

```bash
# This will trigger a SET event notification
SET mykey "Hello World"

# This will trigger a DEL event notification
DEL mykey

# This will trigger an EXPIRED event notification after 5 seconds
SET expiring_key "Will expire" EX 5
```

## 🏗️ Architecture

### Core Components

1. **Configuration Module** (`src/config.rs`)
   - `Config` struct with thread-safe notification settings
   - Environment variable support for default values
   - Runtime configuration changes via CONFIG command

2. **Keyspace Notifications Module** (`src/keyspace_notifications.rs`)
   - `KeyspaceEvent` enum defining event types
   - `KeyspaceNotifications` manager for event publishing
   - Channel name generation following Redis conventions

3. **CONFIG Command** (`src/cmd/config.rs`)
   - `CONFIG GET` to retrieve settings
   - `CONFIG SET` to modify settings
   - `CONFIG LIST` to display all configuration

4. **DEL Command** (`src/cmd/del.rs`)
   - New command for deleting keys
   - Triggers keyspace notifications when keys are removed

5. **Database Integration** (`src/db.rs`)
   - Enhanced `set()` method with SET event notifications
   - New `del()` method with DEL event notifications
   - Background expiration task with EXPIRED event notifications

### Event Flow

```
Database Operation → Check Config → Publish Event → Client Receives
       ↓                    ↓           ↓              ↓
    SET/DEL/EXPIRED → notify_keyspace_events → __keyevent@0__:event → Subscriber
```

### Channel Naming Convention

Following Redis standards, keyspace notification channels use the format:
- `__keyevent@0__:set` - Key creation/update events
- `__keyevent@0__:del` - Key deletion events  
- `__keyevent@0__:expired` - Key expiration events

The `@0` represents database 0 (Mini-Redis only supports single database).

## 🔧 Implementation Details

### Configuration Management

```rust
pub struct Config {
    notify_keyspace_events: Arc<Mutex<bool>>,
}

impl Config {
    pub fn new() -> Self {
        let notify_keyspace_events = env::var("MINI_REDIS_NOTIFY_KEYSPACE_EVENTS")
            .map(|val| val == "1" || val.to_lowercase() == "true")
            .unwrap_or(false);
        
        Self {
            notify_keyspace_events: Arc::new(Mutex::new(notify_keyspace_events)),
        }
    }
}
```

### Event Publishing

```rust
impl KeyspaceNotifications {
    pub fn publish_set(&self, db: &Db, key: &str) {
        if !self.config.notify_keyspace_events() {
            return;
        }
        
        let channel = "__keyevent@0__:set";
        let message = key.to_string();
        let _ = db.publish(channel, Bytes::from(message));
    }
}
```

### Database Integration

```rust
impl Db {
    pub(crate) fn set(&self, key: String, value: Bytes, expire: Option<Duration>) {
        // ... existing set logic ...
        
        // Publish keyspace notification for SET event
        self.shared.keyspace_notifications.publish_set(self, &key);
    }
    
    pub(crate) fn del(&self, key: &str) -> bool {
        if let Some(entry) = state.entries.remove(key) {
            // ... cleanup logic ...
            
            // Publish keyspace notification for DEL event
            self.shared.keyspace_notifications.publish_del(self, key);
            true
        } else {
            false
        }
    }
}
```

## 🧪 Testing

### Running Tests

```bash
# Test configuration functionality
cargo test config

# Test keyspace notifications
cargo test keyspace_notifications

# Test CONFIG command
cargo test cmd::config

# Test DEL command
cargo test cmd::del

# Test the complete system
cargo test
```

### Manual Testing

Use the provided test script:

```bash
./test-keyspace-notifications.sh
```

This script demonstrates:
- CONFIG GET/SET/LIST commands
- Subscription to all event types
- Event generation through SET/DEL/EXP operations
- TTL expiration events
- Dynamic enabling/disabling of notifications

## 📊 Configuration Options

### Environment Variables

- `MINI_REDIS_NOTIFY_KEYSPACE_EVENTS`: Set to "1" or "true" to enable by default

### Runtime Configuration

- `CONFIG SET notify-keyspace-events 1`: Enable notifications
- `CONFIG SET notify-keyspace-events 0`: Disable notifications
- `CONFIG GET notify-keyspace-events`: Check current status

## 🔒 Security Considerations

- Notifications are published to public channels
- No authentication required to subscribe to notification channels
- Consider the information disclosure implications of keyspace events
- Notifications can be disabled to prevent information leakage

## 🚧 Limitations

- Only supports database 0 (no multi-database support)
- Limited to SET, DEL, and EXPIRED events
- No support for other Redis keyspace event types
- Notifications are not persisted across server restarts
- No event filtering or pattern matching

## 🔮 Future Enhancements

Potential improvements could include:

- Support for additional event types (RENAME, EXPIRE, etc.)
- Event filtering and pattern matching
- Multi-database support
- Event persistence and replay
- Performance optimizations for high-frequency events
- Integration with external event systems

## 📚 References

- [Redis Keyspace Notifications](https://redis.io/topics/notifications)
- [Redis CONFIG Command](https://redis.io/commands/config)
- [Redis DEL Command](https://redis.io/commands/del)
- [Redis Pub/Sub](https://redis.io/topics/pubsub)

## 🤝 Contributing

When contributing to keyspace notifications:

1. Ensure all tests pass: `cargo test`
2. Add tests for new functionality
3. Update this documentation
4. Follow the existing code style and patterns
5. Consider performance implications of event publishing
6. Test with both enabled and disabled notification states

## 🎯 Use Cases

Keyspace notifications are useful for:

- **Audit Logging**: Track all database modifications
- **Cache Invalidation**: React to data changes in external caches
- **Real-time Analytics**: Monitor database activity patterns
- **Synchronization**: Keep external systems in sync with database state
- **Debugging**: Understand database usage patterns
- **Metrics Collection**: Build custom monitoring dashboards

---

*This implementation provides Redis-compatible keyspace notifications with configurable event publishing and real-time database event monitoring.* 