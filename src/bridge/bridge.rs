use serenity::async_trait;
use futures::future::join_all;

#[derive(Clone, Debug)]
pub struct MessageBridge {
    username: String,
    avatar: String,
}

#[async_trait]
pub trait Bridge {
    async fn start(&mut self);
    async fn sync(&mut self, message: &MessageBridge);
}

pub struct BridgeManager {
    bridges: Vec<Box<dyn Bridge>>
}

impl BridgeManager {
    pub fn new() -> Self {
        BridgeManager {
            bridges: vec!()
        }
    }

    pub fn registry(&mut self, bridge: Box<dyn Bridge>) {
        self.bridges.push(bridge);
    }

    pub async fn start(mut self) {
        println!("{}座桥已连接", self.bridges.len());
        join_all(self.bridges.iter_mut().map(|bridge| {
            bridge.start()
        })).await;
    }
}
