# 📚 **Mini-Redis Documentation**

Welcome to the comprehensive documentation for the enhanced Mini-Redis project. This directory contains detailed guides for all the new features and enhancements added to the original Mini-Redis implementation.

## 🎯 **What's New**

This branch transforms Mini-Redis from a simple example project into a **production-ready Redis-compatible server** with advanced features including:

- **Extended Command Set** (TTL, INFO, DEL, QUIT)
- **Pattern-Based Pub/Sub** with glob pattern support

- **🧹 Lightweight GC** for background cleanup
- **Complete Monitoring Stack** (Prometheus + Grafana)
- **Production Scripts** for easy deployment and testing

## 📚 **Documentation Index**

This directory contains detailed documentation for all the new features added to Mini-Redis:

- **[MONITORING_README.md](MONITORING_README.md)** - Complete setup guide for Prometheus and Grafana monitoring
- **[PATTERN_PUBSUB_README.md](PATTERN_PUBSUB_README.md)** - Detailed guide for pattern-based Pub/Sub functionality

- **[LIGHTWEIGHT_GC_README.md](LIGHTWEIGHT_GC_README.md)** - Complete guide for the lightweight garbage collection system

## 🚀 **Quick Start**

1. **Clone and Build**: `cargo build --release`
2. **Start Server**: `cargo run --release --bin mini-redis-server -- --metrics-port 9123`
3. **Start Monitoring**: `./setup-complete.sh`
4. **Test Features**: Use the provided test scripts in the root directory

## 🔧 **Key Components**

### **New Commands**
- `TTL` / `PTTL` - Check key expiration times
- `INFO` - Server information and statistics
- `DEL` - Delete keys with notifications
- `QUIT` - Graceful connection termination
- `PSUBSCRIBE` / `PUNSUBSCRIBE` - Pattern-based subscriptions
- `CONFIG` - Runtime server configuration

### **Core Systems**
- **Pattern Matching Engine** - Efficient glob pattern support
- **Keyspace Notifications** - Real-time event publishing
- **Lightweight GC** - Background cleanup system
- **Metrics Server** - Prometheus-compatible endpoint
- **Configuration Management** - Runtime settings control

## 📊 **Monitoring & Observability**

The enhanced Mini-Redis includes a complete monitoring stack:

- **Built-in Metrics**: Operations, memory, keys, pub/sub, GC performance
- **Prometheus Integration**: Standard metrics endpoint at `/metrics`
- **Grafana Dashboards**: Pre-configured visualizations
- **Docker Compose**: Complete monitoring infrastructure

## 🧪 **Testing & Validation**

Comprehensive test scripts are provided for all features:

- `test-pattern-pubsub.sh` - Pattern Pub/Sub functionality
- `test-keyspace-notifications.sh` - Keyspace notifications
- `test-gc.sh` - Lightweight GC system
- `continuous-data.sh` - Load testing and monitoring

## 🎯 **Use Cases**

These enhancements enable:

- **Real-time Applications**: Pattern-based Pub/Sub for dynamic subscriptions
- **Event-Driven Systems**: Keyspace notifications for database changes
- **High-Performance Caching**: Efficient expiration handling with GC
- **Production Monitoring**: Complete observability stack
- **Development & Testing**: Comprehensive Redis-compatible environment

## 🔮 **Future Enhancements**

The architecture is designed for future expansion:

- **Advanced Patterns**: Extended regex support
- **Event Filtering**: Pattern-based event filtering
- **Multi-database**: Support for multiple databases
- **Persistence**: Event and pattern persistence
- **Cluster Support**: Distributed operation

## 🤝 **Contributing**

This project demonstrates modern Rust async patterns and Redis protocol implementation. Contributions are welcome for:

- Performance optimizations
- Additional Redis commands
- Enhanced monitoring capabilities
- Testing improvements
- Documentation updates

---

**🚀 Transform your Redis development experience with this production-ready Mini-Redis implementation!** 