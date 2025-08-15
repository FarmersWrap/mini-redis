use mini_redis::Shutdown;
use tokio::sync::broadcast;
use tokio::time::{self, Duration};
use futures::future::join_all;

#[tokio::test]
async fn shutdown_basic_signal() {
    let (tx, rx) = broadcast::channel(1);
    let mut s = Shutdown::new(rx);
    assert!(!s.is_shutdown());

    let h = tokio::spawn(async move {
        s.recv().await;
        true
    });

    time::sleep(Duration::from_millis(10)).await;
    let _ = tx.send(());
    assert!(h.await.unwrap());
}

#[tokio::test]
async fn shutdown_multiple_waiters() {
    let (tx, rx) = broadcast::channel(1);
    let mut s1 = Shutdown::new(rx);
    let mut s2 = Shutdown::new(tx.subscribe());

    let h1 = tokio::spawn(async move { s1.recv().await; });
    let h2 = tokio::spawn(async move { s2.recv().await; });

    time::sleep(Duration::from_millis(10)).await;
    let _ = tx.send(());

    h1.await.unwrap();
    h2.await.unwrap();
}

#[tokio::test]
async fn shutdown_timeout_no_signal() {
    let (_tx, rx) = broadcast::channel(1);
    let mut s = Shutdown::new(rx);
    let res = tokio::time::timeout(Duration::from_millis(50), s.recv()).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn shutdown_concurrent_waiters() {
    let (tx, rx) = broadcast::channel(1);
    let mut tasks = Vec::new();

    // primary receiver
    tasks.push(tokio::spawn({
        let mut s = Shutdown::new(rx);
        async move { s.recv().await }
    }));

    // many subscribers
    for _ in 0..32 {
        let mut s = Shutdown::new(tx.subscribe());
        tasks.push(tokio::spawn(async move { s.recv().await }));
    }

    time::sleep(Duration::from_millis(10)).await;
    let _ = tx.send(());
    let _ = join_all(tasks).await;
}

#[tokio::test]
async fn shutdown_drop_behavior() {
    let (tx, rx) = broadcast::channel(1);
    let _drop_me = Shutdown::new(rx);
    let mut keep = Shutdown::new(tx.subscribe());
    drop(_drop_me);
    let _ = tx.send(());
    keep.recv().await;
}

#[test]
fn shutdown_debug_fmt() {
    let (_tx, rx) = broadcast::channel(1);
    let s = Shutdown::new(rx);
    let dbg = format!("{:?}", s);
    assert!(!dbg.is_empty());
}
