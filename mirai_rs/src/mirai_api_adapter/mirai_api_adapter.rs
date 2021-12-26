use async_trait::async_trait;

#[async_trait]
pub trait MiraiApiAdapter {
    async fn verify();
}