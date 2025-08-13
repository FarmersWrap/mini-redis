use std::time::Duration;

/// Configuration for the garbage collection (GC) system
#[derive(Debug, Clone)]
pub struct GcConfig {
    /// How often to run GC (default: 250ms)
    pub cleanup_interval: Duration,
    /// Maximum number of expired keys to clean per batch (default: 100)
    pub batch_size: usize,
    /// Whether GC is enabled (default: true)
    pub enabled: bool,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            cleanup_interval: Duration::from_millis(250),
            batch_size: 100,
            enabled: true,
        }
    }
}

impl GcConfig {
    /// Create a new GC configuration with custom settings
    pub fn new(cleanup_interval: Duration, batch_size: usize) -> Self {
        Self {
            cleanup_interval,
            batch_size,
            enabled: true,
        }
    }

    /// Create a configuration with faster cleanup (100ms)
    pub fn fast() -> Self {
        Self {
            cleanup_interval: Duration::from_millis(100),
            batch_size: 50,
            enabled: true,
        }
    }

    /// Create a configuration with slower cleanup (500ms)
    pub fn slow() -> Self {
        Self {
            cleanup_interval: Duration::from_millis(500),
            batch_size: 200,
            enabled: true,
        }
    }

    /// Disable GC
    pub fn disabled() -> Self {
        Self {
            cleanup_interval: Duration::from_millis(250),
            batch_size: 100,
            enabled: false,
        }
    }

    /// Set the cleanup interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.cleanup_interval = interval;
        self
    }

    /// Set the batch size
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Enable or disable GC
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_config_default() {
        let config = GcConfig::default();
        assert_eq!(config.cleanup_interval, Duration::from_millis(250));
        assert_eq!(config.batch_size, 100);
        assert!(config.enabled);
    }

    #[test]
    fn test_gc_config_custom() {
        let config = GcConfig::new(Duration::from_millis(100), 50);
        assert_eq!(config.cleanup_interval, Duration::from_millis(100));
        assert_eq!(config.batch_size, 50);
        assert!(config.enabled);
    }

    #[test]
    fn test_gc_config_fast() {
        let config = GcConfig::fast();
        assert_eq!(config.cleanup_interval, Duration::from_millis(100));
        assert_eq!(config.batch_size, 50);
        assert!(config.enabled);
    }

    #[test]
    fn test_gc_config_slow() {
        let config = GcConfig::slow();
        assert_eq!(config.cleanup_interval, Duration::from_millis(500));
        assert_eq!(config.batch_size, 200);
        assert!(config.enabled);
    }

    #[test]
    fn test_gc_config_disabled() {
        let config = GcConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_gc_config_builder() {
        let config = GcConfig::default()
            .with_interval(Duration::from_millis(150))
            .with_batch_size(75)
            .with_enabled(false);

        assert_eq!(config.cleanup_interval, Duration::from_millis(150));
        assert_eq!(config.batch_size, 75);
        assert!(!config.enabled);
    }
} 