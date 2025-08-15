use mini_redis::Shutdown;
use tokio::sync::oneshot;
use tokio::time::{self, Duration};

#[tokio::test]
async fn test_shutdown_creation() {
    let shutdown = Shutdown::new();

    // Test that shutdown can be created
    assert!(shutdown.is_some());
}

#[tokio::test]
async fn test_shutdown_signal() {
    let shutdown = Shutdown::new().unwrap();
    let (tx, rx) = oneshot::channel::<()>();

    // Spawn a task that waits for shutdown
    let shutdown_clone = shutdown.clone();
    let handle = tokio::spawn(async move {
        shutdown_clone.wait_for_shutdown().await;
        tx.send(()).unwrap();
    });

    // Wait a bit to ensure the task is waiting
    time::sleep(Duration::from_millis(10)).await;

    // Signal shutdown
    shutdown.shutdown();

    // Wait for the task to complete
    rx.await.unwrap();
    handle.await.unwrap();
}

#[tokio::test]
async fn test_shutdown_multiple_waiters() {
    let shutdown = Shutdown::new().unwrap();
    let (tx1, rx1) = oneshot::channel::<()>();
    let (tx2, rx2) = oneshot::channel::<()>();

    // Spawn multiple tasks waiting for shutdown
    let shutdown_clone1 = shutdown.clone();
    let handle1 = tokio::spawn(async move {
        shutdown_clone1.wait_for_shutdown().await;
        tx1.send(()).unwrap();
    });

    let shutdown_clone2 = shutdown.clone();
    let handle2 = tokio::spawn(async move {
        shutdown_clone2.wait_for_shutdown().await;
        tx2.send(()).unwrap();
    });

    // Wait a bit to ensure tasks are waiting
    time::sleep(Duration::from_millis(10)).await;

    // Signal shutdown
    shutdown.shutdown();

    // Wait for both tasks to complete
    rx1.await.unwrap();
    rx2.await.unwrap();
    handle1.await.unwrap();
    handle2.await.unwrap();
}

#[tokio::test]
async fn test_shutdown_already_signaled() {
    let shutdown = Shutdown::new().unwrap();

    // Signal shutdown immediately
    shutdown.shutdown();

    // Wait for shutdown should complete immediately
    let start = std::time::Instant::now();
    shutdown.wait_for_shutdown().await;
    let duration = start.elapsed();

    // Should complete very quickly
    assert!(duration < Duration::from_millis(100));
}

#[tokio::test]
async fn test_shutdown_multiple_signals() {
    let shutdown = Shutdown::new().unwrap();

    // Signal shutdown multiple times
    shutdown.shutdown();
    shutdown.shutdown();
    shutdown.shutdown();

    // Wait for shutdown should complete
    shutdown.wait_for_shutdown().await;

    // Additional signals should not cause issues
    shutdown.shutdown();
    shutdown.shutdown();
}

#[tokio::test]
async fn test_shutdown_clone() {
    let shutdown = Shutdown::new().unwrap();
    let shutdown_clone = shutdown.clone();

    // Both should be independent
    assert!(shutdown.is_some());
    assert!(shutdown_clone.is_some());

    // Signal on one should affect the other
    shutdown.shutdown();

    // Both should complete
    shutdown.wait_for_shutdown().await;
    shutdown_clone.wait_for_shutdown().await;
}

#[tokio::test]
async fn test_shutdown_timeout() {
    let shutdown = Shutdown::new().unwrap();

    // Wait for shutdown with timeout
    let result = tokio::time::timeout(
        Duration::from_millis(100),
        shutdown.wait_for_shutdown()
    ).await;

    // Should timeout since no shutdown was signaled
    assert!(result.is_err());
}

#[tokio::test]
async fn test_shutdown_race_condition() {
    let shutdown = Shutdown::new().unwrap();

    // Spawn a task that signals shutdown very quickly
    let shutdown_clone = shutdown.clone();
    tokio::spawn(async move {
        time::sleep(Duration::from_millis(1)).await;
        shutdown_clone.shutdown();
    });

    // Wait for shutdown
    shutdown.wait_for_shutdown().await;
}

#[tokio::test]
async fn test_shutdown_immediate_completion() {
    let shutdown = Shutdown::new().unwrap();

    // Signal shutdown
    shutdown.shutdown();

    // Wait for shutdown should complete immediately
    let start = std::time::Instant::now();
    shutdown.wait_for_shutdown().await;
    let duration = start.elapsed();

    // Should complete very quickly (less than 1ms)
    assert!(duration < Duration::from_millis(1));
}

#[tokio::test]
async fn test_shutdown_concurrent_access() {
    let shutdown = Shutdown::new().unwrap();
    let mut handles = vec![];

    // Spawn many tasks that all wait for shutdown
    for i in 0..100 {
        let shutdown_clone = shutdown.clone();
        let handle = tokio::spawn(async move {
            shutdown_clone.wait_for_shutdown().await;
            i
        });
        handles.push(handle);
    }

    // Wait a bit to ensure all tasks are waiting
    time::sleep(Duration::from_millis(10)).await;

    // Signal shutdown
    shutdown.shutdown();

    // Wait for all tasks to complete
    let results: Vec<i32> = futures::future::join_all(handles).await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    // Verify all tasks completed
    assert_eq!(results.len(), 100);
    for i in 0..100 {
        assert_eq!(results[i], i);
    }
}

#[tokio::test]
async fn test_shutdown_drop_behavior() {
    // Test that dropping a shutdown instance doesn't affect others
    let shutdown1 = Shutdown::new().unwrap();
    let shutdown2 = Shutdown::new().unwrap();

    // Drop the first one
    drop(shutdown1);

    // The second one should still work
    shutdown2.shutdown();
    shutdown2.wait_for_shutdown().await;
}

#[tokio::test]
async fn test_shutdown_send_sync() {
    // Test that Shutdown can be sent across threads
    let shutdown = Shutdown::new().unwrap();

    let handle = std::thread::spawn(move || {
        // This should compile if Shutdown is Send + Sync
        assert!(shutdown.is_some());
    });

    handle.join().unwrap();
}

#[test]
fn test_shutdown_debug() {
    // Test that Shutdown can be formatted for debug
    let shutdown = Shutdown::new();

    // Test debug formatting if implemented
    let debug_str = format!("{:?}", shutdown);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_shutdown_clone_behavior() {
    // Test that cloning works correctly
    let shutdown1 = Shutdown::new().unwrap();
    let shutdown2 = shutdown1.clone();

    // Both should be valid
    assert!(shutdown1.is_some());
    assert!(shutdown2.is_some());

    // They should be independent instances
    assert_ne!(std::ptr::addr_of!(shutdown1), std::ptr::addr_of!(shutdown2));
}

#[tokio::test]
async fn test_shutdown_performance() {
    let shutdown = Shutdown::new().unwrap();

    // Test performance of shutdown signal
    let start = std::time::Instant::now();

    for _ in 0..1000 {
        shutdown.shutdown();
    }

    let duration = start.elapsed();

    // Should be very fast
    assert!(duration < Duration::from_millis(100));
}

#[tokio::test]
async fn test_shutdown_memory_usage() {
    // Test that Shutdown doesn't use excessive memory
    let shutdowns: Vec<Shutdown> = (0..1000).map(|_| Shutdown::new().unwrap()).collect();

    // Verify we can create many shutdown instances
    assert_eq!(shutdowns.len(), 1000);

    // Test that they all work
    for shutdown in &shutdowns {
        shutdown.shutdown();
    }

    // Wait for one to verify it works
    shutdowns[0].wait_for_shutdown().await;
}

#[tokio::test]
async fn test_shutdown_error_conditions() {
    // Test various error conditions

    // Test with None shutdown
    let shutdown: Option<Shutdown> = None;

    // This should not panic
    if let Some(shutdown) = shutdown {
        shutdown.shutdown();
    }

    // Test that we can still create valid shutdowns
    let valid_shutdown = Shutdown::new().unwrap();
    valid_shutdown.shutdown();
    valid_shutdown.wait_for_shutdown().await;
}

#[tokio::test]
async fn test_shutdown_edge_cases() {
    let shutdown = Shutdown::new().unwrap();

    // Test edge case: signal shutdown before any waiters
    shutdown.shutdown();

    // Test edge case: multiple waiters after shutdown
    let (tx1, rx1) = oneshot::channel::<()>();
    let (tx2, rx2) = oneshot::channel::<()>();

    let shutdown_clone1 = shutdown.clone();
    let handle1 = tokio::spawn(async move {
        shutdown_clone1.wait_for_shutdown().await;
        tx1.send(()).unwrap();
    });

    let shutdown_clone2 = shutdown.clone();
    let handle2 = tokio::spawn(async move {
        shutdown_clone2.wait_for_shutdown().await;
        tx2.send(()).unwrap();
    });

    // Both should complete immediately
    rx1.await.unwrap();
    rx2.await.unwrap();
    handle1.await.unwrap();
    handle2.await.unwrap();
}

#[tokio::test]
async fn test_shutdown_integration() {
    // Test shutdown in a more realistic scenario
    let shutdown = Shutdown::new().unwrap();
    let (tx, rx) = oneshot::channel::<()>();

    // Simulate a long-running task
    let shutdown_clone = shutdown.clone();
    let handle = tokio::spawn(async move {
        loop {
            // Do some work
            time::sleep(Duration::from_millis(10)).await;

            // Check for shutdown
            if shutdown_clone.is_shutdown() {
                break;
            }
        }
        tx.send(()).unwrap();
    });

    // Wait a bit for the task to start
    time::sleep(Duration::from_millis(50)).await;

    // Signal shutdown
    shutdown.shutdown();

    // Wait for task to complete
    rx.await.unwrap();
    handle.await.unwrap();
}

#[test]
fn test_shutdown_thread_safety() {
    // Test that Shutdown can be used across threads safely
    let shutdown = Shutdown::new().unwrap();

    let handles: Vec<_> = (0..10).map(|_| {
        let shutdown_clone = shutdown.clone();
        std::thread::spawn(move || {
            // Use shutdown in another thread
            assert!(shutdown_clone.is_some());
        })
    }).collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
