use everything::{
    knowledge::{Knowledge, Statement},
    model::{Atom, CharacterSet, Object, Set},
};

use crate::{
    StatementChange,
    format::{FileFormatApi, FileFormatApiContract},
};

fn write_and_read(knowledge: &Knowledge) -> Knowledge {
    let mut bytes = Vec::new();

    FileFormatApi::write_knowledge(knowledge, &mut bytes);

    FileFormatApi::read_knowledge(&bytes).expect("the knowledge base must be parseable")
}

fn statement(atom: u128, object: Object) -> Statement {
    Statement {
        atom: Atom(atom),
        object,
    }
}

#[test]
fn round_trips_atoms() {
    let mut knowledge = Knowledge::empty();

    knowledge.add_statement(statement(1, Object::Atom(Atom(2))));
    knowledge.add_statement(statement(1, Object::Atom(Atom(3))));
    knowledge.add_statement(statement(4, Object::Atom(Atom(5))));

    let parsed = write_and_read(&knowledge);

    assert_eq!(parsed.map, knowledge.map);
}

#[test]
fn round_trips_empty_knowledge() {
    let knowledge = Knowledge::empty();

    let parsed = write_and_read(&knowledge);

    assert_eq!(parsed.map, knowledge.map);
}

#[test]
fn round_trips_nested_character_sets() {
    for character in ['A', '\u{DF}', '\u{20AC}', '\u{800}', '\u{1F600}'] {
        let mut knowledge = Knowledge::empty();

        knowledge.add_statement(statement(
            1,
            Object::Set(Set::Character(CharacterSet {
                representing: character,
            })),
        ));

        let parsed = write_and_read(&knowledge);

        assert_eq!(parsed.map, knowledge.map, "character {character:?}");
    }
}

#[test]
fn applies_incremental_changes_after_a_full_write() {
    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(statement(1, Object::Atom(Atom(2))));

    let added = statement(3, Object::Atom(Atom(4)));
    let removed = statement(1, Object::Atom(Atom(2)));

    let mut bytes = Vec::new();
    FileFormatApi::write_knowledge(&knowledge, &mut bytes);
    FileFormatApi::change_knowledge_incremental(
        [
            StatementChange::Add(&added),
            StatementChange::Remove(&removed),
        ]
        .into_iter(),
        &mut bytes,
    );

    let mut expected = knowledge.clone();
    expected.add_statement(added);
    expected.remove_statement(&removed);

    let parsed =
        FileFormatApi::read_knowledge(&bytes).expect("the knowledge base must be parseable");

    assert_eq!(parsed.map, expected.map);
}
