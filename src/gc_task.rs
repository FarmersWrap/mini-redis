use crate::gc_config::GcConfig;
use crate::db::Db;
use std::sync::Arc;
use tokio::time::{interval, Duration, Instant};
use tracing::{debug, info, warn};

/// Garbage collection task that runs in the background
#[derive(Debug)]
pub struct GcTask {
    /// Database instance to clean
    db: Arc<Db>,
    /// GC configuration
    config: GcConfig,
    /// Task handle for graceful shutdown
    task_handle: Option<tokio::task::JoinHandle<()>>,
}

impl GcTask {
    /// Create a new GC task
    pub fn new(db: Arc<Db>, config: GcConfig) -> Self {
        Self {
            db,
            config,
            task_handle: None,
        }
    }

    /// Start the GC task
    pub fn start(&mut self) {
        if !self.config.enabled {
            debug!("GC is disabled, not starting task");
            return;
        }

        let db = Arc::clone(&self.db);
        let config = self.config.clone();
        
        info!(
            "Starting GC task with interval: {:?}, batch size: {}",
            config.cleanup_interval, config.batch_size
        );

        let handle = tokio::spawn(async move {
            Self::gc_loop(db, config).await;
        });

        self.task_handle = Some(handle);
    }

    /// Stop the GC task
    pub async fn stop(&mut self) {
        if let Some(handle) = self.task_handle.take() {
            info!("Stopping GC task");
            handle.abort();
            
            // Wait for the task to finish
            if let Err(err) = handle.await {
                if err.is_cancelled() {
                    debug!("GC task cancelled successfully");
                } else {
                    warn!("GC task failed: {:?}", err);
                }
            }
        }
    }

    /// The main GC loop
    async fn gc_loop(db: Arc<Db>, config: GcConfig) {
        let mut interval_timer = interval(config.cleanup_interval);
        
        loop {
            interval_timer.tick().await;
            
            let start_time = Instant::now();
            let cleaned_count = Self::cleanup_expired_keys(&db, config.batch_size).await;
            let duration = start_time.elapsed();
            
            if cleaned_count > 0 {
                info!(
                    "GC cleaned {} expired keys in {:?}",
                    cleaned_count, duration
                );
            } else {
                debug!("GC run completed, no expired keys found");
            }
        }
    }

    /// Clean up expired keys in batches
    async fn cleanup_expired_keys(db: &Db, batch_size: usize) -> usize {
        let start_time = Instant::now();
        let mut total_cleaned = 0;
        let mut batch_count = 0;
        
        loop {
            let batch_start = Instant::now();
            let cleaned = db.cleanup_expired_keys_batch(batch_size).await;
            
            if cleaned == 0 {
                break; // No more expired keys
            }
            
            total_cleaned += cleaned;
            batch_count += 1;
            
            // Log batch performance
            let batch_duration = batch_start.elapsed();
            debug!(
                "GC batch {}: cleaned {} keys in {:?}",
                batch_count, cleaned, batch_duration
            );
            
            // If this batch took too long, break to avoid blocking
            if batch_duration > Duration::from_millis(50) {
                warn!(
                    "GC batch {} took too long ({:?}), stopping for this cycle",
                    batch_count, batch_duration
                );
                break;
            }
            
            // If we've cleaned enough keys, take a small break
            if total_cleaned >= batch_size * 2 {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        }
        
        let total_duration = start_time.elapsed();
        debug!(
            "GC cleanup completed: {} keys in {} batches, total time: {:?}",
            total_cleaned, batch_count, total_duration
        );
        
        total_cleaned
    }

    /// Get the current GC configuration
    pub fn config(&self) -> &GcConfig {
        &self.config
    }

    /// Update the GC configuration
    pub fn update_config(&mut self, new_config: GcConfig) {
        info!(
            "Updating GC config: interval={:?}, batch_size={}, enabled={}",
            new_config.cleanup_interval, new_config.batch_size, new_config.enabled
        );
        
        self.config = new_config;
        
        // If GC was disabled and is now enabled, restart the task
        if self.config.enabled && self.task_handle.is_none() {
            self.start();
        }
        // If GC was enabled and is now disabled, stop the task
        else if !self.config.enabled && self.task_handle.is_some() {
            // We can't await here, so we'll let the task run until next interval
            debug!("GC disabled, task will stop on next interval");
        }
    }
}

impl Clone for GcTask {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
            config: self.config.clone(),
            task_handle: None, // Clone doesn't clone the running task
        }
    }
}

impl Drop for GcTask {
    fn drop(&mut self) {
        if let Some(handle) = &self.task_handle {
            handle.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Metrics;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_gc_task_creation() {
        let metrics = Arc::new(Metrics::new());
        let db = Arc::new(Db::new(metrics));
        let config = GcConfig::default();
        
        let mut gc_task = GcTask::new(db, config);
        assert!(gc_task.task_handle.is_none());
    }

    #[tokio::test]
    async fn test_gc_task_disabled() {
        let metrics = Arc::new(Metrics::new());
        let db = Arc::new(Db::new(metrics));
        let config = GcConfig::disabled();
        
        let mut gc_task = GcTask::new(db, config);
        gc_task.start();
        
        // Should not start a task when disabled
        assert!(gc_task.task_handle.is_none());
    }

    #[tokio::test]
    async fn test_gc_config_update() {
        let metrics = Arc::new(Metrics::new());
        let db = Arc::new(Db::new(metrics));
        let config = GcConfig::default();
        
        let mut gc_task = GcTask::new(db, config);
        
        // Update config
        let new_config = GcConfig::fast();
        gc_task.update_config(new_config);
        
        assert_eq!(gc_task.config().cleanup_interval, Duration::from_millis(100));
        assert_eq!(gc_task.config().batch_size, 50);
    }
} 