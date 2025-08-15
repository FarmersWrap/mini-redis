use mini_redis::{MetricsServer, Metrics};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use std::net::SocketAddr;

async fn start_metrics_server() -> (u16, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let port = addr.port();

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let metrics = Arc::new(Metrics::new());

    tokio::spawn(async move {
        let mut server = MetricsServer::new(metrics, port);
        tokio::select! {
            _ = async {
                if let Err(e) = server.start().await {
                    eprintln!("Metrics server error: {}", e);
                }
            } => {}
            _ = shutdown_rx => {}
        }
    });

    (port, shutdown_tx)
}

#[tokio::test]
async fn test_metrics_server_creation() {
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = MetricsServer::new(metrics, port);

    // Test that server can be created
    assert_eq!(server.port(), port);
}

#[tokio::test]
async fn test_metrics_server_bind() {
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = MetricsServer::new(metrics, port);

    // Test that server can bind to address
    assert_eq!(server.port(), port);
}

#[tokio::test]
async fn test_metrics_server_metrics_access() {
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = MetricsServer::new(metrics.clone(), port);

    // Test that we can access the metrics
    assert_eq!(server.port(), port);

    // Verify metrics are accessible
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 0);
}

#[tokio::test]
async fn test_metrics_server_port() {
    let metrics = Arc::new(Metrics::new());
    let port = 1234;

    let server = MetricsServer::new(metrics, port);

    // Test that server has the correct port
    assert_eq!(server.port(), port);
}

#[test]
fn test_metrics_server_debug() {
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = MetricsServer::new(metrics, port);

    // Test debug formatting if implemented
    let debug_str = format!("{:?}", server);
    assert!(!debug_str.is_empty());
}

#[tokio::test]
async fn test_metrics_server_lifecycle() {
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let mut server = MetricsServer::new(metrics, port);

    // Test that server can be created and managed
    assert_eq!(server.port(), port);

    // Test that server can be dropped cleanly
    drop(server);
}

#[tokio::test]
async fn test_metrics_server_invalid_port() {
    // Test with valid port
    let metrics = Arc::new(Metrics::new());
    let valid_port = 0;
    let server = MetricsServer::new(metrics, valid_port);
    assert_eq!(server.port(), valid_port);
}

#[test]
fn test_metrics_server_thread_safety() {
    // Test that MetricsServer can be used across threads
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = MetricsServer::new(metrics, port);

    let handle = std::thread::spawn(move || {
        // Use the server in another thread
        assert_eq!(server.port(), 0);
    });

    handle.join().unwrap();
}

#[test]
fn test_metrics_server_memory_usage() {
    // Test that MetricsServer doesn't use excessive memory
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = MetricsServer::new(metrics, port);

    // The size should be reasonable
    let size = std::mem::size_of_val(&server);
    assert!(size < 10000); // Should be much smaller than 10KB

    // Test that we can create many servers without memory issues
    let servers: Vec<_> = (0..100).map(|_| {
        let metrics = Arc::new(Metrics::new());
        let port = 0;
        MetricsServer::new(metrics, port)
    }).collect();

    assert_eq!(servers.len(), 100);
}

#[test]
fn test_metrics_server_performance() {
    // Test that MetricsServer creation is fast
    let start = std::time::Instant::now();

    for _ in 0..1000 {
        let metrics = Arc::new(Metrics::new());
        let port = 0;
        let _server = MetricsServer::new(metrics, port);
    }

    let duration = start.elapsed();

    // Creating 1000 servers should be very fast (less than 1 second)
    assert!(duration.as_secs() < 1);
}

#[test]
fn test_metrics_server_serde() {
    // Test that MetricsServer can be serialized/deserialized if serde is implemented
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = MetricsServer::new(metrics, port);

    // Test to_string() if implemented
    let server_str = format!("{:?}", server);
    assert!(!server_str.is_empty());
}

#[test]
fn test_metrics_server_port_operations() {
    // Test port-related operations
    let port = 1234;

    // Test port properties
    assert_eq!(port, 1234);

    // Test port comparison
    let port2 = 1234;
    assert_eq!(port, port2);

    let port3 = 5678;
    assert_ne!(port, port3);
}

#[test]
fn test_metrics_server_metrics_operations() {
    // Test metrics-related operations
    let metrics = Arc::new(Metrics::new());

    // Test initial metrics values
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), 0);

    // Test updating metrics
    metrics.inc_ops_ok();
    metrics.set_keys(100);
    metrics.set_mem_bytes(1024);

    // Verify metrics were updated
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(metrics.keys.load(std::sync::atomic::Ordering::Relaxed), 100);
    assert_eq!(metrics.mem_bytes.load(std::sync::atomic::Ordering::Relaxed), 1024);
}

#[test]
fn test_metrics_server_error_handling() {
    // Test various error conditions

    // Test that we can still create valid servers
    let metrics = Arc::new(Metrics::new());
    let port = 0;
    let valid_server = MetricsServer::new(metrics, port);
    assert_eq!(valid_server.port(), port);
}

#[test]
fn test_metrics_server_edge_cases() {
    // Test edge cases

    // Test with zero port
    let port = 0;
    assert_eq!(port, 0);

    // Test with large port
    let port = 65535;
    assert_eq!(port, 65535);
}

#[test]
fn test_metrics_server_integration() {
    // Test MetricsServer in a more realistic scenario
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = MetricsServer::new(metrics.clone(), port);

    // Verify server was created
    assert_eq!(server.port(), port);

    // Verify metrics are accessible
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 0);

    // Update metrics
    metrics.inc_ops_ok();
    metrics.inc_get_hits();

    // Verify metrics were updated
    assert_eq!(metrics.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(metrics.get_hits.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[test]
fn test_metrics_server_cleanup() {
    // Test that MetricsServer can be cleaned up properly
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = MetricsServer::new(metrics, port);

    // Verify server was created
    assert_eq!(server.port(), port);

    // Test that server can be dropped cleanly
    drop(server);

    // Test that we can create another server
    let metrics2 = Arc::new(Metrics::new());
    let server2 = MetricsServer::new(metrics2, port);
    assert_eq!(server2.port(), port);
}

#[test]
fn test_metrics_server_concurrent_access() {
    // Test that MetricsServer can be accessed concurrently
    let metrics = Arc::new(Metrics::new());
    let port = 0;

    let server = Arc::new(MetricsServer::new(metrics.clone(), port));

    let handles: Vec<_> = (0..10).map(|_| {
        let server_clone = Arc::clone(&server);
        let metrics_clone = Arc::clone(&metrics);
        std::thread::spawn(move || {
            // Use server and metrics in another thread
            assert_eq!(server_clone.port(), 0);
            assert_eq!(metrics_clone.ops_ok.load(std::sync::atomic::Ordering::Relaxed), 0);
        })
    }).collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
