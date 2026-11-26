use std::sync::LazyLock;

use imbl::OrdMap;

use crate::{
    knowledge::{Knowledge, Properties},
    model::{Atom, Construction},
};

pub static BASE: LazyLock<Knowledge> = LazyLock::new(|| {
    let mut base = OrdMap::new();

    base.insert(
        Atom::NON_CONSTRUCTED_PROPERTY,
        Properties::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );

    base.insert(
        Atom::PROPERTY_CONSTRUCTOR,
        Properties::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );

    // ---

    base.insert(
        Atom::SUCCESSOR_OF,
        Properties::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );
    base.insert(
        Atom::PREDECESSOR_OF,
        Properties::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );

    // ---

    base.insert(
        Atom::ATOM,
        Properties::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );
    base.insert(
        Atom::VALUE,
        Properties::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );

    Knowledge { map: base }
});
