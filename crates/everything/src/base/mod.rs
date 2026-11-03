use std::sync::LazyLock;

use imbl::OrdMap;

use crate::{
    knowledge::{Knowledge, Values},
    model::{Atom, Construction},
};

pub static BASE: LazyLock<Knowledge> = LazyLock::new(|| {
    let mut base = OrdMap::new();

    base.insert(
        Atom::PROPERTY,
        Values::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );

    base.insert(
        Atom::PROPERTY_CONSTRUCTOR,
        Values::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );

    // ---

    base.insert(
        Atom::SUCCESSOR_OF,
        Values::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );
    base.insert(
        Atom::PREDECESSOR_OF,
        Values::from([
            Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into(),
        ]),
    );

    Knowledge { map: base }
});
