use everything_db::Database;

#[tokio::main]
async fn main() {
    let db = Database::new("test.evdb".into()).await.unwrap();

    // db.save().await.unwrap();

    println!("{:?}", db.root);
}
