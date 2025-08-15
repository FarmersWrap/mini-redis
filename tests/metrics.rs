use mini_redis::Metrics;
use std::sync::Arc;

#[test]
fn test_metrics_creation() {
    let metrics = Metrics::new();

    // Test initial values
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.ops_err.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.get_misses.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.pub_count.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.sub_count.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), 0);
}

#[test]
fn test_metrics_ops_counting() {
    let metrics = Metrics::new();

    // Test incrementing operations
    metrics.inc_ops_ok();
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(metrics.ops_err.load(std::sync::atomic::Ordering::Relaxed), 0);

    metrics.inc_ops_ok();
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 2);

    metrics.inc_ops_err();
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 2);
    assert_eq!(metrics.ops_err.load(std::sync::atomic::Ordering::Relaxed), 1);

    metrics.inc_ops_err();
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 2);
    assert_eq!(metrics.ops_err.load(std::sync::atomic::Ordering::Relaxed), 2);
}

#[test]
fn test_metrics_get_ops() {
    let metrics = Metrics::new();

    // Test get operations
    metrics.inc_get_hits();
    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 1);

    metrics.inc_get_hits();
    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 2);

    // Test get hits and misses
    metrics.inc_get_hits();
    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 3);

    metrics.inc_get_misses();
    assert_eq!(metrics.get_misses.load(std::sync::atomic::Ordering::Relaxed), 1);

    metrics.inc_get_hits();
    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 4);

    metrics.inc_get_misses();
    assert_eq!(metrics.get_misses.load(std::sync::atomic::Ordering::Relaxed), 2);
}

#[test]
fn test_metrics_pub_sub() {
    let metrics = Metrics::new();

    // Test publish operations
    metrics.inc_pub_count();
    assert_eq!(metrics.pub_count.load(std::sync::atomic::Ordering::Relaxed), 1);

    metrics.inc_pub_count();
    assert_eq!(metrics.pub_count.load(std::sync::atomic::Ordering::Relaxed), 2);

    // Test subscribe operations
    metrics.inc_sub_count();
    assert_eq!(metrics.sub_count.load(std::sync::atomic::Ordering::Relaxed), 1);

    metrics.inc_sub_count();
    assert_eq!(metrics.sub_count.load(std::sync::atomic::Ordering::Relaxed), 2);

    metrics.inc_sub_count();
    assert_eq!(metrics.sub_count.load(std::sync::atomic::Ordering::Relaxed), 3);
}

#[test]
fn test_metrics_keys_and_memory() {
    let metrics = Metrics::new();

    // Test setting keys count
    metrics.set_keys(100);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 100);

    metrics.set_keys(250);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 250);

    // Test setting memory usage
    metrics.set_mem_bytes(1024);
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), 1024);

    metrics.set_mem_bytes(2048);
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), 2048);

    // Test with large values
    metrics.set_keys(1_000_000);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 1_000_000);

    metrics.set_mem_bytes(1_073_741_824); // 1GB
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), 1_073_741_824);
}

#[test]
fn test_metrics_thread_safety() {
    let metrics = Arc::new(Metrics::new());
    let mut handles = vec![];

    // Spawn multiple threads to test concurrent access
    for _i in 0..10 {
        let metrics_clone = Arc::clone(&metrics);
        let handle = std::thread::spawn(move || {
            for i in 0..100 {
                metrics_clone.inc_ops_ok();
                metrics_clone.inc_get_hits();
                if i % 2 == 0 {
                    metrics_clone.inc_get_hits();
                } else {
                    metrics_clone.inc_get_misses();
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify the final counts
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 1000);
    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 1500);
    assert_eq!(metrics.get_misses.load(std::sync::atomic::Ordering::Relaxed), 500);
}

#[test]
fn test_metrics_clone() {
    let metrics = Metrics::new();

    // Set some values
    metrics.inc_ops_ok();
    metrics.inc_get_hits();
    metrics.set_keys(50);

    // Clone the metrics
    let cloned_metrics = metrics.clone();

    // Verify both have the same values
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(cloned_metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 1);

    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(cloned_metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 1);

    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 50);
    assert_eq!(cloned_metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 50);

    // Modify the original
    metrics.inc_ops_ok();

    // Verify they are independent
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 2);
    assert_eq!(cloned_metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[test]
fn test_metrics_debug() {
    let metrics = Metrics::new();

    // Set some values
    metrics.inc_ops_ok();
    metrics.inc_get_hits();
    metrics.set_keys(100);

    // Test debug formatting
    let debug_str = format!("{:?}", metrics);

    // Verify debug string contains key information
    assert!(debug_str.contains("ops_ok"));
    assert!(debug_str.contains("get_hits"));
    assert!(debug_str.contains("keys"));
}

#[test]
fn test_metrics_edge_cases() {
    let metrics = Metrics::new();

    // Test with maximum values
    metrics.set_keys(u64::MAX);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), u64::MAX);

    metrics.set_mem_bytes(u64::MAX);
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), u64::MAX);

    // Test with very large numbers
    let large_number = 1_000_000_000_000_u64;
    metrics.set_keys(large_number);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), large_number);

    metrics.set_mem_bytes(large_number);
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), large_number);
}

#[test]
fn test_metrics_operations_chain() {
    let metrics = Metrics::new();

    // Test chaining multiple operations
    metrics.inc_ops_ok();
    metrics.inc_get_hits();
    metrics.inc_get_hits();
    metrics.inc_pub_count();
    metrics.inc_sub_count();

    // Verify all operations were applied
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 2);
    assert_eq!(metrics.pub_count.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(metrics.sub_count.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[test]
fn test_metrics_negative_scenarios() {
    let metrics = Metrics::new();

    // Test that metrics don't go negative (they shouldn't)
    // This test documents the expected behavior

    // Set to zero
    metrics.set_keys(0);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 0);

    metrics.set_mem_bytes(0);
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), 0);

    // Verify counters start at 0
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.ops_err.load(std::sync::atomic::Ordering::Relaxed), 0);
}

#[test]
fn test_metrics_performance() {
    let metrics = Metrics::new();

    // Test performance with many operations
    let start = std::time::Instant::now();

    for _ in 0..1_000_000 {
        metrics.inc_ops_ok();
    }

    let duration = start.elapsed();

    // Verify all operations were counted
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 1_000_000);

    // Performance should be reasonable (less than 1 second for 1M operations)
    assert!(duration.as_secs() < 1);
}

#[test]
fn test_metrics_serialization() {
    let metrics = Metrics::new();

    // Set some values
    metrics.inc_ops_ok();
    metrics.inc_get_hits();
    metrics.set_keys(100);
    metrics.set_mem_bytes(1024);

    // Test that metrics can be converted to string representation
    let ops_total = metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed).to_string();
    let keys = metrics.keys.load(std::sync::atomic::Ordering::Relaxed).to_string();
    let mem_bytes = metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed).to_string();

    assert_eq!(ops_total, "1");
    assert_eq!(keys, "100");
    assert_eq!(mem_bytes, "1024");
}

#[test]
fn test_metrics_info_string() {
    let metrics = Metrics::new();

    // Set some values
    metrics.inc_ops_ok();
    metrics.inc_get_hits();
    metrics.set_keys(100);
    metrics.set_mem_bytes(1024);

    // Test info string format
    let info = metrics.info_string();

    // Verify info string contains key metrics
    assert!(info.contains("ops_ok:1"));
    assert!(info.contains("get_hits:1"));
    assert!(info.contains("keys:100"));
    assert!(info.contains("mem_bytes:1024"));
}

#[test]
fn test_metrics_prometheus_string() {
    let metrics = Metrics::new();

    // Set some values
    metrics.inc_ops_ok();
    metrics.inc_get_hits();
    metrics.set_keys(100);
    metrics.set_mem_bytes(1024);

    // Test Prometheus format
    let prometheus = metrics.prometheus_string();

    // Verify Prometheus string contains key metrics
    assert!(prometheus.contains("mini_redis_ops_ok 1"));
    assert!(prometheus.contains("mini_redis_get_hits 1"));
    assert!(prometheus.contains("mini_redis_keys 100"));
    assert!(prometheus.contains("mini_redis_mem_bytes 1024"));
}

#[test]
fn test_metrics_gc_operations() {
    let metrics = Metrics::new();

    // Test GC operations
    metrics.inc_gc_cleanup_count();
    assert_eq!(metrics.gc_cleanup_count.load(std::sync::atomic::Ordering::Relaxed), 1);

    metrics.add_gc_cleanup_total(50);
    assert_eq!(metrics.gc_cleanup_total.load(std::sync::atomic::Ordering::Relaxed), 50);

    metrics.add_gc_cleanup_total(25);
    assert_eq!(metrics.gc_cleanup_total.load(std::sync::atomic::Ordering::Relaxed), 75);

    // Test duration recording
    let duration = std::time::Duration::from_millis(100);
    metrics.record_gc_duration(duration);

    assert_eq!(metrics.gc_duration_count.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(metrics.gc_duration_total.load(std::sync::atomic::Ordering::Relaxed), 100);
}

#[test]
fn test_metrics_default() {
    let metrics = Metrics::default();

    // Test that default creates the same as new
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.ops_err.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.get_misses.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.pub_count.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.sub_count.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), 0);
}
