// use mirai_rs::{self, Mirai};
use websocket::ClientBuilder;

mod bridge;

use bridge::MessageBridge;

// mod dc_demo;

#[tokio::main]
async fn main() {
    let MessageBridge = MessageBridge::new();
    // Mirai::new();
    // websocket::Client
    let mut client = ClientBuilder::new("ws://localhost:8080/all?verifyKey=INITKEYKPRGCLwL&qq=3245538509")
    .unwrap()
    .connect_insecure()
    .unwrap();

    // for message in client.incoming_messages() {
    //     println!("{:?}", message)
    // }
    


    	// Construct a new connection to the websocket server
	// let runner = ClientBuilder::new("ws://localhost:8080/all?verifyKey=INITKEYKPRGCLwL&qq=3245538509")
    // .unwrap()
    // .async_connect_insecure();

    // let a = *runner;
    // a.await;
}
