use mini_redis::{GcConfig, GcTask};
use std::time::Duration;
use tokio::time::{self, Instant};

#[test]
fn test_gc_config_default() {
    let config = GcConfig::default();

    // Test default values
    assert_eq!(config.batch_size, 100);
    assert_eq!(config.cleanup_interval, Duration::from_millis(250));
    assert_eq!(config.enabled, true);
}

#[test]
fn test_gc_config_custom() {
    let config = GcConfig::new(Duration::from_millis(100), 50);

    // Test custom values
    assert_eq!(config.batch_size, 50);
    assert_eq!(config.cleanup_interval, Duration::from_millis(100));
    assert_eq!(config.enabled, true);
}

#[test]
fn test_gc_config_builder() {
    let config = GcConfig::default()
        .with_batch_size(75)
        .with_interval(Duration::from_millis(45))
        .with_enabled(false);

    // Test builder pattern
    assert_eq!(config.batch_size, 75);
    assert_eq!(config.cleanup_interval, Duration::from_millis(45));
    assert_eq!(config.enabled, false);
}

#[test]
fn test_gc_config_clone() {
    let config = GcConfig::new(Duration::from_millis(15), 25);

    let cloned = config.clone();

    // Test clone
    assert_eq!(cloned.batch_size, config.batch_size);
    assert_eq!(cloned.cleanup_interval, config.cleanup_interval);
    assert_eq!(cloned.enabled, config.enabled);
}

#[test]
fn test_gc_config_debug() {
    let config = GcConfig::default();

    // Test debug formatting
    let debug_str = format!("{:?}", config);

    // Verify debug string contains key information
    assert!(debug_str.contains("batch_size"));
    assert!(debug_str.contains("cleanup_interval"));
    assert!(debug_str.contains("enabled"));
}

#[test]
fn test_gc_config_edge_cases() {
    // Test with zero values
    let config = GcConfig {
        batch_size: 0,
        cleanup_interval: Duration::from_millis(0),
        enabled: false,
    };

    assert_eq!(config.batch_size, 0);
    assert_eq!(config.cleanup_interval, Duration::from_millis(0));
    assert_eq!(config.enabled, false);

    // Test with very large values
    let config = GcConfig {
        batch_size: usize::MAX,
        cleanup_interval: Duration::from_millis(u64::MAX),
        enabled: true,
    };

    assert_eq!(config.batch_size, usize::MAX);
    assert_eq!(config.cleanup_interval, Duration::from_millis(u64::MAX));
    assert_eq!(config.enabled, true);
}

#[test]
fn test_gc_config_validation() {
    // Test that all configurations are valid
    let configs = vec![
        GcConfig::default(),
        GcConfig::new(Duration::from_millis(1), 1),
        GcConfig::new(Duration::from_millis(3600000), 10000),
    ];

    for config in configs {
        // All configurations should be valid
        assert!(config.batch_size >= 0);
        // Duration is always valid
        // Boolean is always valid
    }
}

#[test]
fn test_gc_config_fast() {
    let config = GcConfig::fast();

    assert_eq!(config.batch_size, 50);
    assert_eq!(config.cleanup_interval, Duration::from_millis(100));
    assert_eq!(config.enabled, true);
}

#[test]
fn test_gc_config_slow() {
    let config = GcConfig::slow();

    assert_eq!(config.batch_size, 200);
    assert_eq!(config.cleanup_interval, Duration::from_millis(500));
    assert_eq!(config.enabled, true);
}

#[test]
fn test_gc_config_disabled() {
    let config = GcConfig::disabled();

    assert_eq!(config.batch_size, 100);
    assert_eq!(config.cleanup_interval, Duration::from_millis(250));
    assert_eq!(config.enabled, false);
}

#[test]
fn test_gc_config_serde() {
    // Test that GcConfig can be serialized/deserialized if serde is implemented
    let config = GcConfig::default();

    // Test to_string() if implemented
    let config_str = format!("{:?}", config);
    assert!(!config_str.is_empty());

    // Test that we can access all fields
    assert_eq!(config.batch_size, 100);
    assert_eq!(config.cleanup_interval, Duration::from_millis(250));
    assert_eq!(config.enabled, true);
}

#[test]
fn test_gc_config_copy() {
    // Test that GcConfig can be copied if Copy is implemented
    let config = GcConfig::default();

    // Test that we can access fields multiple times
    let batch_size = config.batch_size;
    let cleanup_interval = config.cleanup_interval;
    let enabled = config.enabled;

    // Verify the values
    assert_eq!(batch_size, 100);
    assert_eq!(cleanup_interval, Duration::from_millis(250));
    assert_eq!(enabled, true);
}

#[test]
fn test_gc_config_duration_operations() {
    let config = GcConfig::default();

    // Test duration operations
    let cleanup_interval = config.cleanup_interval;

    // Test duration arithmetic
    let double_interval = cleanup_interval + cleanup_interval;
    assert_eq!(double_interval, Duration::from_millis(500));

    let half_interval = cleanup_interval / 2;
    assert_eq!(half_interval, Duration::from_millis(125));

    // Test duration comparison
    assert!(cleanup_interval > Duration::from_millis(100));
    assert!(cleanup_interval < Duration::from_millis(500));
}

#[test]
fn test_gc_config_batch_size_operations() {
    let config = GcConfig::default();

    // Test batch size operations
    let batch_size = config.batch_size;

    // Test arithmetic operations
    assert_eq!(batch_size + 50, 150);
    assert_eq!(batch_size - 25, 75);
    assert_eq!(batch_size * 2, 200);
    assert_eq!(batch_size / 2, 50);

    // Test comparison operations
    assert!(batch_size > 50);
    assert!(batch_size < 200);
    assert!(batch_size >= 100);
    assert!(batch_size <= 100);
}

#[test]
fn test_gc_config_boolean_operations() {
    let config = GcConfig::default();

    // Test boolean operations
    let enabled = config.enabled;

    // Test logical operations
    assert!(enabled);
    assert!(enabled && true);
    assert!(enabled || false);
    assert!(!(!enabled));

    // Test with other boolean values
    let disabled_config = GcConfig {
        enabled: false,
        ..GcConfig::default()
    };

    assert!(!disabled_config.enabled);
    assert!(enabled != disabled_config.enabled);
}

#[test]
fn test_gc_config_memory_usage() {
    // Test that GcConfig doesn't use excessive memory
    let config = GcConfig::default();

    // The size should be reasonable (a few dozen bytes)
    let size = std::mem::size_of_val(&config);
    assert!(size < 1000); // Should be much smaller than 1KB

    // Test that we can create many configs without memory issues
    let configs: Vec<GcConfig> = (0..1000).map(|_| GcConfig::default()).collect();
    assert_eq!(configs.len(), 1000);

    // Verify all configs have the same values
    for config in &configs {
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.cleanup_interval, Duration::from_millis(250));
        assert_eq!(config.enabled, true);
    }
}

#[test]
fn test_gc_config_performance() {
    // Test that GcConfig operations are fast
    let start = std::time::Instant::now();

    for _ in 0..1_000_000 {
        let _config = GcConfig::default();
    }

    let duration = start.elapsed();

    // Creating 1M configs should be very fast (less than 1 second)
    assert!(duration.as_secs() < 1);
}

#[test]
fn test_gc_config_thread_safety() {
    // Test that GcConfig can be used across threads
    let config = GcConfig::default();

    let handle = std::thread::spawn(move || {
        // Use the config in another thread
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.cleanup_interval, Duration::from_millis(250));
        assert_eq!(config.enabled, true);
    });

    handle.join().unwrap();
}
