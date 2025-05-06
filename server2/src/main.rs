use std::sync::Arc;
use tracing_subscriber::fmt::init;
use server2::Database;
use server2::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init();
    
    let db = Arc::new(Database::open("D:\\Everything".as_ref()).await?);
    println!("Sequence: {}", db.sequence());
    
    Ok(())
}