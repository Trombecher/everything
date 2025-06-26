use tracing_subscriber::fmt::init;
use everything::objects::core::ROWS;

#[tokio::main]
async fn main() -> () {
    init();
    
    println!("{:#?}", ROWS)
}