use everything::objects::core::ROWS;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();

    for row in ROWS.iter() {
        println!("{:?}", row.clone().decode());
    }
}