use crate::bridge::{Bridge, MessageBridge};
use serenity::async_trait;
use tokio::time;

pub struct QQBridge {
    messages: Vec<MessageBridge>
}

impl QQBridge {
    pub fn builder() -> QQBridge {
        QQBridge {
            messages: vec!()
        }
    }


    async fn syncToQQ(&self) {
        loop {
            if let Some(message) = self.messages.get(0) {
                println!("{:?}", message);
                println!("计划在这里同步消息qq");
            }
            time::sleep(time::Duration::from_millis(200)).await;
        }
    }
}

#[async_trait]
impl Bridge for QQBridge {

    async fn sync(&mut self, message: &MessageBridge) {
        self.messages.push(message.clone());
    }

    async fn start(&mut self) {
        println!("开始连接QQ");
        tokio::select! {
            _ = self.syncToQQ() => {
                println!("discord 的同步消息 不应该走到这里");
            }
            _ = super::QQConnection::start() => {
                println!("qq error");
            }
        };
    }

}

