use mini_redis::clients::Client;
use mini_redis::server;

use bytes::Bytes;
use futures::future::join_all;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::timeout;

async fn start_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move { server::run(listener, tokio::signal::ctrl_c(), None).await });

    addr
}

#[tokio::test]
async fn stress_many_short_lived_connections() {
    let addr = start_server().await;

    let num_clients = 50usize;

    let tasks = (0..num_clients).map(|i| {
        let addr = addr;
        tokio::spawn(async move {
            let mut client = Client::connect(addr).await.unwrap();
            // Alternate ping and small set/gets
            if i % 2 == 0 {
                let pong = client.ping(None).await.unwrap();
                assert_eq!(&pong[..], b"PONG");
            } else {
                let key = format!("k{}", i);
                let val = format!("v{}", i);
                client.set(&key, Bytes::from(val.clone())).await.unwrap();
                let got = client.get(&key).await.unwrap().unwrap();
                assert_eq!(&got[..], val.as_bytes());
            }
        })
    });

    timeout(Duration::from_secs(10), join_all(tasks))
        .await
        .expect("stress_many_short_lived_connections timed out")
        .into_iter()
        .for_each(|res| res.unwrap());
}

#[tokio::test]
async fn stress_pubsub_fanout() {
    let addr = start_server().await;

    // Create multiple subscribers on the same channel
    let channel = "fanout".to_string();
    let subscriber_count = 5usize;
    let message_count = 5usize;

    let mut subs = Vec::with_capacity(subscriber_count);
    for _ in 0..subscriber_count {
        let client = Client::connect(addr).await.unwrap();
        let sub = client.subscribe(vec![channel.clone()]).await.unwrap();
        subs.push(sub);
    }

    // Publish messages and assert reported subscriber count
    let publishers = (0..message_count).map(|i| {
        let addr = addr;
        let channel = channel.clone();
        tokio::spawn(async move {
            let mut client = Client::connect(addr).await.unwrap();
            let n = client
                .publish(&channel, Bytes::from(format!("m{}", i)))
                .await
                .unwrap();
            assert!(n >= 1);
            n
        })
    });

    // Collect publisher results
    let results = timeout(Duration::from_secs(10), join_all(publishers))
        .await
        .expect("publishers timed out");
    for r in results {
        r.unwrap();
    }

    // Each subscriber should receive all messages
    let recv_tasks = subs.into_iter().map(|mut sub| {
        let channel_value = channel.clone();
        tokio::spawn(async move {
            for i in 0..message_count {
                let msg = timeout(Duration::from_secs(2), sub.next_message())
                    .await
                    .expect("subscriber recv timeout")
                    .unwrap()
                    .unwrap();
                assert_eq!(msg.channel, channel_value);
                assert_eq!(&msg.content[..], format!("m{}", i).as_bytes());
            }
        })
    });

    timeout(Duration::from_secs(10), join_all(recv_tasks))
        .await
        .expect("subscribers timed out")
        .into_iter()
        .for_each(|res| res.unwrap());
}

#[tokio::test]
async fn stress_mixed_workload() {
    let addr = start_server().await;

    // Single subscriber to ensure publish returns >=1 and messages are delivered
    let client = Client::connect(addr).await.unwrap();
    let mut subscriber = client.subscribe(vec!["load".into()]).await.unwrap();

    let workers = 10usize;
    let ops_per_worker = 20usize;

    // Compute expected number of publishes given the modulo scheduling below
    let expected_publishes: usize = (0..workers)
        .map(|wi| (0..ops_per_worker).filter(|op| (wi + op) % 3 == 1).count())
        .sum();

    let pub_task = tokio::spawn(async move {
        for _ in 0..expected_publishes {
            let msg = timeout(Duration::from_secs(3), subscriber.next_message())
                .await
                .expect("subscriber blocked")
                .unwrap()
                .unwrap();
            assert_eq!(msg.channel, "load");
            // Do not assert exact content to avoid ordering constraints
            assert!(!msg.content.is_empty());
        }
    });

    let tasks = (0..workers).map(|wi| {
        let addr = addr;
        tokio::spawn(async move {
            let mut client = Client::connect(addr).await.unwrap();
            for op in 0..ops_per_worker {
                // Round-robin operations
                match (wi + op) % 3 {
                    0 => {
                        let key = format!("k:{}:{}", wi, op);
                        let val = format!("v:{}:{}", wi, op);
                        client.set(&key, Bytes::from(val.clone())).await.unwrap();
                        let got = client.get(&key).await.unwrap().unwrap();
                        assert_eq!(&got[..], val.as_bytes());
                    }
                    1 => {
                        let n = client.publish("load", Bytes::from(format!("p:{}:{}", wi, op))).await.unwrap();
                        assert!(n >= 1);
                    }
                    _ => {
                        let key = format!("k:{}:{}", wi, op);
                        // Best-effort get; may be None if not set yet
                        let _ = client.get(&key).await.unwrap();
                    }
                }
            }
        })
    });

    timeout(Duration::from_secs(20), join_all(tasks))
        .await
        .expect("workers timed out")
        .into_iter()
        .for_each(|res| res.unwrap());

    timeout(Duration::from_secs(10), pub_task)
        .await
        .expect("subscriber aggregator timed out")
        .unwrap();
}

#[tokio::test]
async fn stress_large_payloads_concurrent() {
    let addr = start_server().await;

    let clients = 10usize;
    let payload = Bytes::from(vec![b'x'; 50_000]);

    let tasks = (0..clients).map(|i| {
        let addr = addr;
        let payload = payload.clone();
        tokio::spawn(async move {
            let mut client = Client::connect(addr).await.unwrap();
            let key = format!("blob:{}", i);
            client.set(&key, payload.clone()).await.unwrap();
            let got = client.get(&key).await.unwrap().unwrap();
            assert_eq!(&got[..], &payload[..]);
        })
    });

    timeout(Duration::from_secs(20), join_all(tasks))
        .await
        .expect("large payload tasks timed out")
        .into_iter()
        .for_each(|res| res.unwrap());
}


