fn main() {
    println!("Hello, world!");
}


pub struct Mirai {

}

impl Mirai {
    pub fn new() -> Self {
        Mirai {  }
    }
}

pub mod mirai_api_adapter;

pub use mirai_api_adapter::MiraiApiAdapter;
pub use mirai_api_adapter::ws_adapter::WsAdapter;
