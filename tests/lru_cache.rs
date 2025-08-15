use mini_redis::{server, Connection, Frame};
use bytes::Bytes;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

async fn start_server() -> (String, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        server::run(listener, async move {
            let _ = shutdown_rx.await;
        }, None)
        .await;
    });

    (format!("{}:{}", addr.ip(), addr.port()), shutdown_tx)
}

async fn send_cmd(conn: &mut Connection, parts: Vec<Frame>) -> Frame {
    let frame = Frame::Array(parts);
    conn.write_frame(&frame).await.unwrap();
    let resp = conn.read_frame().await.unwrap().unwrap();
    resp
}

async fn connect(addr: &str) -> Connection {
    let stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    Connection::new(stream)
}

#[tokio::test(flavor = "current_thread")]
async fn test_lru_cache_with_config() {
    let (addr, shutdown) = start_server().await;
    let mut conn = connect(&addr).await;

    // Set max keys to 3
    let resp = send_cmd(
        &mut conn,
        vec![
            Frame::Bulk(Bytes::from("config")),
            Frame::Bulk(Bytes::from("set")),
            Frame::Bulk(Bytes::from("maxkeys")),
            Frame::Bulk(Bytes::from("3")),
        ],
    )
    .await;
    assert!(matches!(resp, Frame::Bulk(ref s) if s == "OK"));

    // Verify max keys is set
    let resp = send_cmd(
        &mut conn,
        vec![
            Frame::Bulk(Bytes::from("config")),
            Frame::Bulk(Bytes::from("get")),
            Frame::Bulk(Bytes::from("maxkeys")),
        ],
    )
    .await;
    assert!(matches!(resp, Frame::Bulk(ref s) if s == "3"));

    // Set 4 keys (should trigger LRU eviction)
    for i in 1..=4 {
        let resp = send_cmd(
            &mut conn,
            vec![
                Frame::Bulk(Bytes::from("set")),
                Frame::Bulk(Bytes::from(format!("key{}", i))),
                Frame::Bulk(Bytes::from(format!("value{}", i))),
            ],
        )
        .await;
        assert!(matches!(resp, Frame::Simple(ref s) if s == "OK"));
    }

    // First key should be evicted (LRU)
    let resp = send_cmd(
        &mut conn,
        vec![
            Frame::Bulk(Bytes::from("get")),
            Frame::Bulk(Bytes::from("key1")),
        ],
    )
    .await;
    assert!(matches!(resp, Frame::Null));

    // Other keys should still exist
    for i in 2..=4 {
        let resp = send_cmd(
            &mut conn,
            vec![
                Frame::Bulk(Bytes::from("get")),
                Frame::Bulk(Bytes::from(format!("key{}", i))),
            ],
        )
        .await;
        assert!(matches!(resp, Frame::Bulk(ref s) if *s == format!("value{}", i)));
    }

    // Access key2 to make it most recently used
    let resp = send_cmd(
        &mut conn,
        vec![
            Frame::Bulk(Bytes::from("get")),
            Frame::Bulk(Bytes::from("key2")),
        ],
    )
    .await;
    assert!(matches!(resp, Frame::Bulk(ref s) if *s == "value2"));

    // Add another key - should evict key3 (least recently used)
    let resp = send_cmd(
        &mut conn,
        vec![
            Frame::Bulk(Bytes::from("set")),
            Frame::Bulk(Bytes::from("key5")),
            Frame::Bulk(Bytes::from("value5")),
        ],
    )
    .await;
    assert!(matches!(resp, Frame::Simple(ref s) if s == "OK"));

    // key3 should be evicted
    let resp = send_cmd(
        &mut conn,
        vec![
            Frame::Bulk(Bytes::from("get")),
            Frame::Bulk(Bytes::from("key3")),
        ],
    )
    .await;
    assert!(matches!(resp, Frame::Null));

    // key2, key4, key5 should still exist
    for i in [2, 4, 5] {
        let resp = send_cmd(
            &mut conn,
            vec![
                Frame::Bulk(Bytes::from("get")),
                Frame::Bulk(Bytes::from(format!("key{}", i))),
            ],
        )
        .await;
        assert!(matches!(resp, Frame::Bulk(ref s) if *s == format!("value{}", i)));
    }

    let _ = shutdown.send(());
}

#[tokio::test(flavor = "current_thread")]
async fn test_config_list_includes_maxkeys() {
    let (addr, shutdown) = start_server().await;
    let mut conn = connect(&addr).await;

    let resp = send_cmd(
        &mut conn,
        vec![
            Frame::Bulk(Bytes::from("config")),
            Frame::Bulk(Bytes::from("list")),
        ],
    )
    .await;

    // Should return an array with config descriptions
    match resp {
        Frame::Array(items) => {
            let descriptions: Vec<String> = items
                .iter()
                .filter_map(|item| {
                    if let Frame::Bulk(bytes) = item {
                        String::from_utf8(bytes.to_vec()).ok()
                    } else {
                        None
                    }
                })
                .collect();

            // Should include maxkeys description
            assert!(descriptions.iter().any(|desc| desc.contains("maxkeys")));
        }
        _ => panic!("Expected array response from CONFIG LIST"),
    }

    let _ = shutdown.send(());
}
