use websocket::ClientBuilder;
use tokio::time;
use websocket::futures::{Future, Stream, Sink};
use websocket::Message;

pub struct QQConnection;

impl QQConnection {
    pub async fn start() {
        let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
        // .worker_threads(1)
        // .enable_all()
        // .build()
        // .unwrap();

        let client = ClientBuilder::new("CONNECTION")
		.unwrap()
		.add_protocol("rust-websocket")
		.async_connect_insecure();
        // let mut client =
        //     ClientBuilder::new("ws://localhost:8080/all?verifyKey=INITKEYKPRGCLwL&qq=3245538509")
        //         .unwrap()
        //         .async_connect_insecure()
        //         .map(|(m, _)| {
        //             println!("接收到qq消息2");
        //             // assert_eq!(m, Some(Message::text("hallo").into()));
        //             false
        //         });
        // tokio::join!(client);
        // runtime.block_on(client);
    }
}
