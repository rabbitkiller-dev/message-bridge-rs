use async_trait::async_trait;
use crate::MiraiApiAdapter;

pub struct WsAdapter {
    
}

#[async_trait]
impl MiraiApiAdapter for WsAdapter {

    async fn verify() {

    }
}