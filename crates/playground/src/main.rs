use everything::{
    Knowledge,
    base::BASE,
    ext::{ObjectExt, StructureExt},
};
use everything_structures::{Object, Property, Structure};

fn main() {
    const GAY: Object = Object::Abstract(34543579348);
    const DRIVES_A_MIATA: Object = Object::Abstract(5345345898);
    const COOL: Object = Object::Abstract(453853495834);

    const DAVID: Object = Object::Abstract(99910934);

    let knowledge = Knowledge::new(
        BASE.union(&Structure::new_set([
            Structure::new_statement(GAY, Object::AXIOMATIC, Object::from_bool(true)).into(),
            Structure::new_statement(DRIVES_A_MIATA, Object::AXIOMATIC, Object::from_bool(true))
                .into(),
            Structure::new_statement(
                COOL,
                Object::COMPUTED,
                Structure::new_node_and([
                    Structure::new_node_exists(
                        Structure::new(&mut [
                            Property {
                                tag: Object::STATEMENT_SUBJECT,
                                value: Structure::new_node_parameter(0).into(),
                            },
                            Property {
                                tag: Object::STATEMENT_TAG,
                                value: GAY,
                            },
                        ])
                        .into(),
                    )
                    .into(),
                    Structure::new_node_exists(
                        Structure::new(&mut [
                            Property {
                                tag: Object::STATEMENT_SUBJECT,
                                value: Structure::new_node_parameter(0).into(),
                            },
                            Property {
                                tag: Object::STATEMENT_TAG,
                                value: DRIVES_A_MIATA,
                            },
                        ])
                        .into(),
                    )
                    .into(),
                ])
                .into(),
            )
            .into(),
            Structure::new_statement(DAVID, GAY, Structure::EMPTY.into()).into(),
            Structure::new_statement(DAVID, DRIVES_A_MIATA, Structure::EMPTY.into()).into(),
        ])),
    )
    .unwrap();

    let qr = knowledge.query_values(&DAVID, COOL);

    if qr.collect_to_set().is_truthy() {
        println!("David is cool")
    } else {
        println!("David is not cool")
    }
}
