use core2::{Database, ids::PackedId};

fn main() {
    let db = Database::from_file("./test.db".as_ref()).unwrap();

    println!("{:?}", db.version());
}
