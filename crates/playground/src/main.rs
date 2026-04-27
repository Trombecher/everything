use everything::{
    Knowledge,
    base::BASE,
    ext::{AbstractExt, StructureExt},
};
use everything_structures::{Abstract, Object, Structure};

/*
tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
    .unwrap();
 */

fn main() {
    const ITEMS_IN_HAND: Object = Object::Abstract(Abstract(4253458394));
    const ITEMS_IN_STORAGE: Object = Object::Abstract(Abstract(5345334950));
    const TOTAL_ITEMS: Object = Object::Abstract(Abstract(4594353099000191));
    const PLAYER: Object = Object::Abstract(Abstract(58439809209999));

    let knowledge = Knowledge::new(
        BASE.union(&Structure::new_set([
            Structure::new_statement(
                ITEMS_IN_HAND,
                Abstract::AXIOMATIC.into(),
                Structure::new_bool(true).into(),
            )
            .into(),
            Structure::new_statement(
                ITEMS_IN_STORAGE,
                Abstract::AXIOMATIC.into(),
                Structure::new_bool(true).into(),
            )
            .into(),
            Structure::new_statement(
                TOTAL_ITEMS,
                Abstract::COMPUTED.into(),
                Structure::new_node_add(
                    Structure::new_node_query_values(
                        Structure::new_node_parameter(0).into(),
                        ITEMS_IN_HAND,
                    )
                    .into(),
                    Structure::new_node_query_values(
                        Structure::new_node_parameter(0).into(),
                        ITEMS_IN_STORAGE,
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
            Structure::new_statement(PLAYER, ITEMS_IN_HAND, Object::new_natural_number(42)).into(),
            Structure::new_statement(PLAYER, ITEMS_IN_STORAGE, Object::new_natural_number(67))
                .into(),
        ])),
    )
    .unwrap();

    let total_items = knowledge.query_values(&PLAYER, TOTAL_ITEMS);

    for item in total_items.values() {
        println!("{:?}", item)
    }
}
