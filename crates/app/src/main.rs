use everything::{Object, Structure};

fn main() {
    println!(
        "{:?}",
        Object::Structure(Structure::new(&mut [])).cmp(&Object::Structure(Structure::new(&mut [])))
    )
}
