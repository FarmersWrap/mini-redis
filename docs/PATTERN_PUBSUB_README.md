# Pattern-Based Pub/Sub (PSUBSCRIBE, PUNSUBSCRIBE)

This document describes the implementation of pattern-based Pub/Sub functionality in Mini-Redis, which extends the standard Pub/Sub system with glob pattern matching capabilities.

## 🎯 Overview

Pattern-based Pub/Sub allows clients to subscribe to multiple channels using glob-style patterns, enabling efficient subscription to groups of related channels without needing to know their exact names in advance.

## ✨ Features

- **Glob Pattern Support**: `*` for any sequence, `?` for single character
- **Efficient Matching**: Compiled regex patterns for performance
- **Multiple Patterns**: Subscribe to multiple patterns simultaneously
- **Redis Compatible**: Follows Redis PSUBSCRIBE protocol
- **Real-time Updates**: Dynamic subscription management

## 🚀 Usage Examples

### Basic Pattern Matching

```bash
# Subscribe to all news channels
PSUBSCRIBE news.*

# Subscribe to user channels with single character wildcard
PSUBSCRIBE user:?123

# Subscribe to multiple patterns
PSUBSCRIBE updates:* notifications:*
```

### Pattern Examples

| Pattern | Matches | Doesn't Match |
|---------|---------|---------------|
| `news.*` | `news.sports`, `news.tech`, `news.local` | `sports.news`, `news` |
| `user:?123` | `user:a123`, `user:b123` | `user:123`, `user:ab123` |
| `updates:*` | `updates.system`, `updates.user` | `system.updates` |
| `*:status` | `user:status`, `system:status` | `status:user` |

### Publishing to Matching Channels

```bash
# These will match news.* pattern
PUBLISH news.sports "Sports update"
PUBLISH news.tech "Tech news"
PUBLISH news.local "Local news"

# These will match user:?123 pattern
PUBLISH user:a123 "User A message"
PUBLISH user:b123 "User B message"
```

## 🏗️ Architecture

### Core Components

1. **Pattern Module** (`src/pattern.rs`)
   - `Pattern` struct for compiled glob patterns
   - Efficient regex-based matching
   - Thread-safe pattern compilation

2. **PSubscribe Command** (`src/cmd/psubscribe.rs`)
   - `PSUBSCRIBE` command implementation
   - Pattern subscription management
   - Message routing to pattern subscribers

3. **Database Integration** (`src/db.rs`)
   - Pattern subscription storage
   - Message fan-out to matching patterns
   - Subscription lifecycle management

### Pattern Compilation

```rust
impl Pattern {
    pub fn new(pattern: &str) -> Result<Self, regex::Error> {
        let regex_str = pattern
            .chars()
            .map(|c| match c {
                '*' => ".*".to_string(),
                '?' => ".".to_string(),
                '.' => "\\.".to_string(),
                // ... other special characters
                _ => c.escape_default().to_string(),
            })
            .collect::<String>();
        
        let regex = regex::Regex::new(&format!("^{}$", regex_str))?;
        Ok(Pattern { pattern: pattern.to_string(), regex })
    }
}
```

### Message Routing

```rust
impl Db {
    pub(crate) fn publish(&self, key: &str, value: Bytes) -> usize {
        let mut total_recipients = 0;
        
        // Send to exact channel subscribers
        if let Some(sender) = state.pub_sub.get(key) {
            let _ = sender.send(value.clone());
            total_recipients += sender.receiver_count();
        }
        
        // Send to pattern subscribers
        for (pattern, sender) in &state.pattern_subscriptions {
            if pattern.matches(key) {
                let _ = sender.send(value.clone());
                total_recipients += sender.receiver_count();
            }
        }
        
        total_recipients
    }
}
```

## 🔧 Implementation Details

### Pattern Storage

Pattern subscriptions are stored in the database state alongside regular channel subscriptions:

```rust
pub struct State {
    // ... existing fields
    pattern_subscriptions: Vec<(Pattern, broadcast::Sender<Bytes>)>,
}
```

### Subscription Management

Each pattern subscription creates a new broadcast channel for message distribution:

```rust
pub(crate) fn psubscribe(&self, pattern: String) -> broadcast::Receiver<Bytes> {
    let pattern = Pattern::new(&pattern)?;
    let (tx, rx) = broadcast::channel(1);
    
    state.pattern_subscriptions.push((pattern, tx));
    Ok(rx)
}
```

### Message Fan-out

When publishing, messages are sent to both exact channel subscribers and pattern subscribers:

1. **Exact Match**: Send to subscribers of the exact channel name
2. **Pattern Match**: Send to all pattern subscribers whose pattern matches the channel name
3. **Efficiency**: Each message is cloned only for active pattern subscribers

## 🧪 Testing

### Running Tests

```bash
# Test pattern matching functionality
cargo test pattern

# Test PSubscribe command
cargo test cmd::psubscribe

# Test the complete system
cargo test
```

### Manual Testing

Use the provided test script:

```bash
./test-pattern-pubsub.sh
```

This script demonstrates:
- Pattern subscription with various glob patterns
- Message publishing to matching channels
- Pattern matching behavior
- Subscription cleanup

## 📊 Performance Characteristics

### Pattern Compilation
- **Compile Time**: O(n) where n is pattern length
- **Memory**: Minimal overhead per pattern
- **Caching**: Patterns are compiled once and reused

### Message Routing
- **Time Complexity**: O(p) where p is number of active patterns
- **Memory**: O(1) per message (cloning only for active subscribers)
- **Scalability**: Efficient for hundreds of patterns

### Subscription Management
- **Add/Remove**: O(1) amortized
- **Lookup**: O(p) for message routing
- **Storage**: Linear with number of active patterns

## 🔒 Security Considerations

- **Pattern Validation**: Patterns are validated during compilation
- **Resource Limits**: No built-in limits on pattern complexity
- **Memory Usage**: Each pattern creates a broadcast channel
- **DoS Protection**: Consider limiting pattern complexity in production

## 🚧 Limitations

- **Single Database**: Only supports database 0
- **Pattern Types**: Limited to glob-style patterns
- **Complexity**: No support for regex or advanced pattern matching
- **Persistence**: Patterns are not persisted across server restarts

## 🔮 Future Enhancements

Potential improvements could include:

- **Advanced Patterns**: Regex support, character classes
- **Pattern Groups**: Logical operations on patterns
- **Performance**: Pattern indexing for faster matching
- **Monitoring**: Pattern usage metrics and analytics
- **Limits**: Configurable pattern complexity limits

## 📚 References

- [Redis PSUBSCRIBE](https://redis.io/commands/psubscribe)
- [Redis PUNSUBSCRIBE](https://redis.io/commands/punsubscribe)
- [Redis Pub/Sub](https://redis.io/topics/pubsub)
- [Glob Patterns](https://en.wikipedia.org/wiki/Glob_(programming))

## 🤝 Contributing

When contributing to pattern Pub/Sub:

1. **Test Coverage**: Ensure all patterns are tested
2. **Performance**: Consider impact on message routing
3. **Documentation**: Update examples and usage patterns
4. **Edge Cases**: Handle special characters and edge cases
5. **Backward Compatibility**: Maintain Redis protocol compatibility

---

*This implementation provides Redis-compatible pattern-based Pub/Sub with efficient glob pattern matching and real-time message routing.* 