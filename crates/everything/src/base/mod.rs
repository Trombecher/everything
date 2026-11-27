use std::sync::LazyLock;

use imbl::OrdMap;
use num_bigint::BigInt;

use crate::{
    knowledge::{Knowledge, Properties},
    model::{Atom, Composite, Construction, Object},
};

fn property_constraint(for_property_constructor: Atom) -> Object {
    // contruction_values (subject, for_property_constructor)
    let construction_values: Object = Composite::from([
        Construction::new(
            Atom::NODE_CONSTRUCTION_VALUES_OBJECT.into(),
            Composite::from([Construction::new(
                Atom::NODE_PARAMETER.into(),
                Object::from(BigInt::ONE),
            )
            .into()])
            .into(),
        )
        .into(),
        Construction::new(
            Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR.into(),
            Composite::from([Construction::new(
                Atom::NODE_LITERAL.into(),
                for_property_constructor.into(),
            )
            .into()])
            .into(),
        )
        .into(),
    ])
    .into();

    // |construction_values| == 1
    let equal = Composite::from([
        Construction::new(
            Atom::NODE_EQUAL_FIRST.into(),
            Composite::from([
                Construction::new(Atom::NODE_SET_COUNT.into(), construction_values).into(),
            ])
            .into(),
        )
        .into(),
        Construction::new(
            Atom::NODE_EQUAL_SECOND.into(),
            Composite::from([Construction::new(
                Atom::NODE_LITERAL.into(),
                Object::from(BigInt::ONE),
            )
            .into()])
            .into(),
        )
        .into(),
    ])
    .into();

    // TODO: add check that value is a node; but for that we need
    // a node that checks if an object is a node (which would
    // be very slow to implement in raw Everything).

    Composite::from([Construction::new(
        Atom::NODE_FUNCTION.into(),
        Composite::from([Construction::new(Atom::NODE_FUNCTION.into(), equal).into()]).into(),
    )
    .into()])
    .into()
}

pub static BASE: LazyLock<Knowledge> = LazyLock::new(|| {
    let mut base = OrdMap::new();

    // --- Absolute base of Everything ---

    base.insert(
        Atom::NON_CONSTRUCTED_PROPERTY,
        Properties::from([Construction::new(
            Atom::PROPERTY_CONSTRUCTOR.into(),
            property_constraint(Atom::NON_CONSTRUCTED_PROPERTY),
        )
        .into()]),
    );

    base.insert(
        Atom::PROPERTY_CONSTRUCTOR,
        Properties::from([Construction::new(
            Atom::PROPERTY_CONSTRUCTOR.into(),
            property_constraint(Atom::PROPERTY_CONSTRUCTOR),
        )
        .into()]),
    );

    // --- Statements for the computation system to work ---

    // TODO: this is very incomplete, we need to add all
    // (non) constructed properties which are either constructed
    // by the computation system at runtime or for which there
    // are specializations in-memory.
    //
    // The following declarations are incomplete.

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
