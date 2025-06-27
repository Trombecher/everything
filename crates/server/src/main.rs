use std::{error::Error, path::Path, sync::Arc};

use everything::{db::Database, objects, query::Demanded};
use tracing::{Level, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt::fmt()
            .with_max_level(Level::DEBUG)
            .finish(),
    )?;

    info!("test");

    let db = Arc::new(Database::new(Box::from(Path::new("./main.everythingdb"))).await?);

    let mut values = db.query((objects::core::TAG, objects::core::TAG_SCHEMA, Demanded));

    println!("{:?}", values.next().is_some());

    Ok(())
}
