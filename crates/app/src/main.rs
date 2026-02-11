use basic::Database;

fn main() {
    let db = Database::try_from();

    println!("{:?}", db.version());
}
