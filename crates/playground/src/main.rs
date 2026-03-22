use std::fs;

use everything::{Knowledge, base::BASE};
use everything_structures::{Object, Structure};
use everything_structures_ff::parse_structure;

fn main() {
    const PERSON: Object = Object::Abstract(530495834059348);

    let content = fs::read_to_string("example.struct").unwrap();
    let knowledge_extension = parse_structure(&content).unwrap();
    let knowledge = Knowledge::new(BASE.union(&knowledge_extension)).unwrap();

    for subject in knowledge.query_subjects_axiomatically(PERSON, Structure::EMPTY.into()) {
        println!("{subject:?}")
    }
}
