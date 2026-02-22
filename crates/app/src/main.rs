use std::sync::Arc;

use everything::{Object, Structure};

fn main() {
    println!(
        "{:?}",
        Object::Structure(Structure(Arc::from([])))
            .cmp(&Object::Structure(Structure(Arc::from([]))))
    )
}
