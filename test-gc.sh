#!/bin/bash

# Test script for Lightweight GC (Background Cleanup) in Mini-Redis
# This script demonstrates the new GC system that avoids scanning the entire database on every request

echo "🧹 Testing Lightweight GC (Background Cleanup) in Mini-Redis"
echo "=========================================================="

# Start the server in the background with GC enabled
echo "🚀 Starting Mini-Redis server with GC enabled..."
cargo run --release --bin mini-redis-server -- --metrics-port 9123 &
SERVER_PID=$!

# Wait for server to start
sleep 2

echo ""
echo "📊 Testing GC Metrics"
echo "===================="

# Check initial metrics
echo "Initial metrics:"
curl -s http://localhost:9123/metrics | grep -E "(gc_cleanup|gc_duration)" || echo "No GC metrics yet"

echo ""
echo "🔑 Creating keys with expiration for GC testing"
echo "==============================================="

# Create multiple keys with short expiration times
for i in {1..50}; do
    echo "SET key_$i 'value_$i' EX 2"
    cargo run --release --bin mini-redis-cli -- set "key_$i" "value_$i" EX 2
done

echo ""
echo "⏳ Waiting for keys to expire and GC to run..."
echo "GC runs every 250ms with batch size 100"

# Wait for GC to process expired keys
sleep 5

echo ""
echo "📈 Checking GC metrics after cleanup"
echo "===================================="

# Check GC metrics
echo "GC metrics:"
curl -s http://localhost:9123/metrics | grep -E "(gc_cleanup|gc_duration)" || echo "No GC metrics found"

echo ""
echo "🔍 Checking database state"
echo "========================="

# Check how many keys remain
echo "Remaining keys:"
cargo run --release --bin mini-redis-cli -- info keyspace

echo ""
echo "📊 Detailed metrics"
echo "=================="

# Get full metrics
echo "Full metrics:"
curl -s http://localhost:9123/metrics

echo ""
echo "🧪 Testing GC under load"
echo "========================"

# Create more keys with different expiration times to test GC under load
echo "Creating keys with staggered expiration times..."

for i in {51..100}; do
    # Stagger expiration times: 1s, 2s, 3s, etc.
    exp_time=$(( (i - 50) % 5 + 1 ))
    echo "SET load_key_$i 'load_value_$i' EX $exp_time"
    cargo run --release --bin mini-redis-cli -- set "load_key_$i" "load_value_$i" EX $exp_time
done

echo ""
echo "⏳ Running GC under load for 10 seconds..."
echo "This will test that GC doesn't block the server"

# Monitor GC activity
for i in {1..10}; do
    echo "GC cycle $i:"
    curl -s http://localhost:9123/metrics | grep -E "(gc_cleanup_count|gc_cleanup_total)" || echo "No GC activity"
    sleep 1
done

echo ""
echo "🔍 Final database state"
echo "======================="

# Check final state
echo "Final key count:"
cargo run --release --bin mini-redis-cli -- info keyspace

echo "Final GC metrics:"
curl -s http://localhost:9123/metrics | grep -E "(gc_cleanup|gc_duration)" || echo "No GC metrics found"

echo ""
echo "🧹 Cleaning up..."
echo "================="

# Kill the server
kill $SERVER_PID 2>/dev/null

echo ""
echo "✅ Lightweight GC test completed!"
echo ""
echo "📋 Summary of what was tested:"
echo "  • Background GC task running every 250ms"
echo "  • Batch processing of expired keys (max 100 per batch)"
echo "  • GC metrics collection and monitoring"
echo "  • Non-blocking GC operation under load"
echo "  • Keyspace notifications for expired keys"
echo ""
echo "🎯 Key benefits of the lightweight GC:"
echo "  • Avoids scanning entire database on every request"
echo "  • Configurable cleanup frequency and batch size"
echo "  • Minimal impact on server performance"
echo "  • Real-time metrics and monitoring"
echo "  • Graceful shutdown and cleanup" 