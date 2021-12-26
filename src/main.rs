// use mirai_rs::{self, Mirai};

mod bridge;
mod discord;
mod qq;

use bridge::{BridgeManager, Bridge};

use std::sync::{Arc, Mutex};
use tokio::runtime::Builder;
use tokio::sync::mpsc;

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut manager = BridgeManager::new();
    let bridge = Box::new(discord::DiscordBridge::builder("OTEyMDE3OTM3MTE4OTQ1MzYx.YZp05A.hzgPVz-fHkP-WV_gCb5GdKdyzok"));
    let qq_bridge = Box::new(qq::QQBridge::builder());
    manager.registry(qq_bridge);
    manager.registry(bridge);
    runtime.block_on(manager.start());
}



#[test]
fn test_join_all() {
    let runtime = Builder::new_multi_thread()
    .worker_threads(1)
    .enable_all()
    .build()
    .unwrap();

    runtime.block_on(async {
        futures::future::join_all(vec![1, 2, 3].iter().map(|i| {
            let i = i.clone();
            tokio::spawn(async move {
                loop {
                    println!("{} 走", i);
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                }
            })
        })).await
    });
}

#[test]
fn test_qq() {
    let runtime = Builder::new_multi_thread()
    .worker_threads(1)
    .enable_all()
    .build()
    .unwrap();
    // use websocket::ClientBuilder;
    // let mut client = ClientBuilder::new("ws://localhost:8080/all?verifyKey=INITKEYKPRGCLwL&qq=3245538509")
    // .unwrap()
    // .connect_insecure()
    // .unwrap();

    // for message in client.incoming_messages() {
    //     println!("{:?}", message)
    // }
    runtime.block_on(async {
        use hyper::body::Buf;
        use hyper::{Client, Request, Method, Body};
        use serde_json::{Result, Value};
        use serde::{Deserialize, Serialize};
        let client = Client::new();
        let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:8080/verify")
        .body(Body::from(r#"{"verifyKey": "INITKEYKPRGCLwL"}"#))
        .unwrap();
        let mut res = client.request(req).await.unwrap();
        println!("{:?}", res);
            // asynchronously aggregate the chunks of the body
        let body = hyper::body::aggregate(res).await.unwrap();

        // try to parse as json with serde_json
        let val: VerifyRes = serde_json::from_reader(body.reader()).unwrap();
       
        // while let Some(next) = res.data().await {
        //     println!("q");
        //     let chunk = next?;
        //     io::stdout().write_all(&chunk).await?;
        // }
        #[derive(Debug, Deserialize)]
        struct VerifyRes {
            code: i32,
            session: String,
        }
        println!("{:?}", val);
    })
}