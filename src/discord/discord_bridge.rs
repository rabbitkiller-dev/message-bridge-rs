use crate::bridge::{Bridge, MessageBridge};
use serenity::{
    async_trait,
    prelude::*,
};
use tokio::time;

pub struct DiscordBridge {
    messages: Vec<MessageBridge>,
    // queue: Queue<MessageBridge>,
    token: String,
}

impl DiscordBridge {
    pub fn builder(token: &str) -> DiscordBridge {
        DiscordBridge {
            messages: vec!(),
            token: token.to_string()
        }
    }


    async fn syncToDiscord(&self) {
        loop {
            if let Some(message) = self.messages.get(0) {
                println!("{:?}", message);
                println!("计划在这里同步消息");
            }
            time::sleep(time::Duration::from_millis(200)).await;
        }
    }
}

#[async_trait]
impl Bridge for DiscordBridge {

    async fn sync(&mut self, message: &MessageBridge) {
        self.messages.push(message.clone());
    }

    async fn start(&mut self) {
        println!("开始连接Discord");
        let mut client = Client::builder(&self.token).event_handler(super::DiscordConnection).await.expect("Err creating client");
        tokio::select! {
            _ = self.syncToDiscord() => {
                println!("discord 的同步消息 不应该走到这里");
            }
            Err(why) = client.start() => {
                println!("Client error: {:?}", why);
            }
        };
    }

}


#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
}

impl <T> Queue<T> {
    fn new() -> Self {
        Queue{ qdata: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.qdata.push(item);
    }

    fn dequeue(&mut self) ->Option<T> {
        let l = self.qdata.len();

        if l > 0 {
            let v = self.qdata.remove(0);
            Some(v)
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.qdata.len()
    }
}
