use std::fs;

use everything_structures_ff::parse_structure;

fn main() {
    let content = fs::read_to_string("example.struct").unwrap();
    let knowledge_extension = parse_structure(&content).unwrap();

    println!("{knowledge_extension:?}");
}
