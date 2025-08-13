# Monitoring with Prometheus and Grafana

This document describes the monitoring setup for Mini-Redis using Prometheus for metrics collection and Grafana for visualization.

## 🎯 Overview

Mini-Redis includes a complete monitoring stack that provides real-time visibility into server performance, operations, and resource usage. The monitoring system consists of:

- **Built-in Metrics Server**: Prometheus-compatible endpoint in Mini-Redis
- **Prometheus**: Time-series database for metrics collection
- **Grafana**: Visualization platform with pre-configured dashboards

## 🚀 Quick Start

### Start Everything at Once

```bash
# Start all services (recommended)
./setup-complete.sh
```

### Start Services Individually

```bash
# Start monitoring stack (Prometheus + Grafana)
./start-monitoring.sh

# Start Mini-Redis with metrics enabled
./run-mini-redis.sh
```

### Access the Services

- **Mini-Redis**: localhost:6379
- **Metrics Endpoint**: http://localhost:9123/metrics
- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (admin/admin)

### Stop Everything

```bash
./stop-all.sh
```

## 📊 Metrics Available

### Core Metrics

- **Operations Counters**:
  - `mini_redis_ops_ok` - Successful operations
  - `mini_redis_ops_err` - Failed operations
  - `mini_redis_get_hits` - GET command hits
  - `mini_redis_get_misses` - GET command misses

- **Resource Usage**:
  - `mini_redis_keys` - Total number of keys
  - `mini_redis_mem_bytes` - Memory usage in bytes

- **Pub/Sub Metrics**:
  - `mini_redis_pub_count` - Published messages
  - `mini_redis_sub_count` - Active subscriptions

### Metrics Endpoint

The metrics are exposed at `/metrics` in Prometheus format:

```bash
curl http://localhost:9123/metrics
```

Example output:
```
# HELP mini_redis_ops_ok Total successful operations
# TYPE mini_redis_ops_ok counter
mini_redis_ops_ok 42

# HELP mini_redis_keys Total number of keys
# TYPE mini_redis_keys gauge
mini_redis_keys 15

# HELP mini_redis_mem_bytes Memory usage in bytes
# TYPE mini_redis_mem_bytes gauge
mini_redis_mem_bytes 2048
```

## 🏗️ Architecture

### Components

1. **Mini-Redis Metrics Server** (`src/metrics_server.rs`)
   - HTTP server on port 9123
   - Prometheus-compatible metrics endpoint
   - Real-time metrics collection

2. **Prometheus** (`prometheus/`)
   - Configuration for scraping Mini-Redis metrics
   - Time-series data storage
   - Query language (PromQL) support

3. **Grafana** (`dashboards/`)
   - Pre-configured dashboards
   - Auto-provisioning of datasources
   - Real-time visualization

### Data Flow

```
Mini-Redis → Metrics Server → Prometheus → Grafana
     ↓              ↓            ↓          ↓
  Operations → /metrics → Scraping → Dashboards
```

## 🔧 Configuration

### Prometheus Configuration

Located in `prometheus/prometheus.yml`:

```yaml
global:
  scrape_interval: 15s
  scrape_timeout: 10s

scrape_configs:
  - job_name: 'mini-redis'
    static_configs:
      - targets: ['host.docker.internal:9123']
    scrape_interval: 10s
    scrape_timeout: 5s
    honor_labels: true
```

### Grafana Configuration

Located in `dashboards/`:

- **Datasource**: `dashboards/datasources/prometheus.yml`
- **Dashboard**: `dashboards/mini-redis-dashboard.json`
- **Provisioning**: `dashboards/provisioning/dashboards.yml`

### Docker Compose

Located in `docker-compose.yml`:

```yaml
services:
  prometheus:
    image: prom/prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus:/etc/prometheus
      - prometheus_data:/prometheus

  grafana:
    image: grafana/grafana
    ports:
      - "3000:3000"
    volumes:
      - ./dashboards:/etc/grafana/provisioning
      - grafana_data:/var/lib/grafana
```

## 📈 Dashboard Features

### Mini-Redis Dashboard

The pre-configured dashboard includes:

- **Operations Overview**: Success/error rates, throughput
- **Resource Usage**: Memory consumption, key counts
- **Pub/Sub Activity**: Message publishing, subscription counts
- **Performance Metrics**: Response times, operation counts

### Dashboard Panels

1. **Operations Summary**
   - Total operations (success + errors)
   - Operation success rate
   - GET hit/miss ratio

2. **Resource Monitoring**
   - Memory usage over time
   - Key count trends
   - Memory per key

3. **Pub/Sub Metrics**
   - Published message count
   - Active subscriptions
   - Message throughput

## 🧪 Testing

### Generate Test Data

```bash
# Generate continuous test data
./continuous-data.sh

# Or use the test script
./test-pattern-pubsub.sh
```

### Verify Metrics

```bash
# Check metrics endpoint
curl http://localhost:9123/metrics | grep mini_redis

# Check Prometheus targets
curl http://localhost:9090/api/v1/targets

# Check Grafana datasources
curl -u admin:admin http://localhost:3000/api/datasources
```

## 🔍 Troubleshooting

### Common Issues

1. **No Data in Grafana**
   - Check Prometheus targets: http://localhost:9090/targets
   - Verify Mini-Redis metrics: http://localhost:9123/metrics
   - Check Docker container status

2. **Prometheus Connection Issues**
   - Verify `host.docker.internal` resolves correctly
   - Check Mini-Redis is running on port 6379
   - Ensure metrics server is enabled

3. **Grafana Login Issues**
   - Default credentials: admin/admin
   - Check container logs: `docker logs <grafana-container>`
   - Verify volume mounts

### Debugging Commands

```bash
# Check container status
docker ps

# View container logs
docker logs <container-name>

# Check port bindings
netstat -tlnp | grep -E ':(6379|9090|3000|9123)'

# Test Mini-Redis connection
cargo run --bin mini-redis-cli ping
```

## 📊 Performance Considerations

### Metrics Collection

- **Scrape Interval**: 10s for Mini-Redis, 15s for Prometheus
- **Memory Impact**: Minimal overhead from metrics collection
- **Network**: Local metrics collection, no external dependencies

### Storage

- **Prometheus**: Local time-series storage
- **Retention**: Configurable via Prometheus settings
- **Backup**: Consider volume persistence for production

## 🔒 Security

### Access Control

- **Metrics Endpoint**: No authentication (localhost only)
- **Prometheus**: No authentication (localhost only)
- **Grafana**: Default admin/admin credentials

### Production Considerations

- **Network Security**: Restrict access to monitoring ports
- **Authentication**: Enable Prometheus/Grafana authentication
- **TLS**: Use HTTPS for external access
- **Firewall**: Block external access to monitoring ports

## 🚀 Production Deployment

### Scaling Considerations

- **Multiple Instances**: Use load balancer for Mini-Redis
- **Metrics Aggregation**: Consider Prometheus federation
- **High Availability**: Grafana clustering for redundancy
- **Backup Strategy**: Regular Prometheus data backups

### Monitoring Best Practices

- **Alerting**: Set up Grafana alerts for critical metrics
- **Logging**: Correlate metrics with application logs
- **Capacity Planning**: Monitor trends for resource planning
- **Performance Tuning**: Use metrics to optimize operations

---

*This monitoring setup provides comprehensive visibility into Mini-Redis performance and operations with minimal configuration overhead.* 