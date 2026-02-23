use everything::{Abstract, Knowledge, Object, Property, Statement, Structure};

fn main() {
    let empty_structure = Object::Structure(Structure::new(&mut []));

    let accept_all = Object::Structure(Structure::new(&mut [Property {
        tag: Object::NODE_FUNCTION,
        value: Object::Structure(Structure::new(&mut [Property {
            tag: Object::NODE_FUNCTION,
            value: Object::Structure(Structure::new(&mut [Property {
                tag: Object::NODE_LITERAL,
                value: empty_structure.clone(),
            }])),
        }])),
    }]));

    let person = Object::Abstract(Abstract("Person".into()));
    let david = Object::Abstract(Abstract("David".into()));

    let mut statements = [
        Statement {
            target: person.clone(),
            tag: Object::AXIOMATIC,
            value: accept_all,
        },
        Statement {
            target: david,
            tag: person,
            value: empty_structure,
        },
    ];

    let knowledge = Knowledge::new(&mut statements);

    println!("{:?}", knowledge)
}
