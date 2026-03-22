use std::fs;

use everything::{
    Knowledge,
    base::BASE,
    ext::{ObjectExt, StructureExt},
};
use everything_structures_ff::parse_structure;

fn main() {
    let content = fs::read_to_string("example.struct").unwrap();
    let knowledge_extension = parse_structure(&content).unwrap();
    let knowledge = Knowledge::new(BASE.union(&knowledge_extension)).unwrap();
}
