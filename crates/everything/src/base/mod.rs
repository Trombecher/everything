use std::sync::LazyLock;

use imbl::OrdMap;

use crate::{
    knowledge::Knowledge,
    model::{Atom, Construction, Set},
};

pub static BASE: LazyLock<Knowledge> = LazyLock::new(|| {
    let mut base = OrdMap::new();

    base.insert(
        Atom::PROPERTY,
        Set::from([Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into()]),
    );

    base.insert(
        Atom::PROPERTY_CONSTRUCTOR,
        Set::from([Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into()]),
    );

    // ---

    base.insert(
        Atom::SUCCESSOR_OF,
        Set::from([Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into()]),
    );
    base.insert(
        Atom::PREDECESSOR_OF,
        Set::from([Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), Atom::TRUE.into()).into()]),
    );

    Knowledge { map: base }
});
