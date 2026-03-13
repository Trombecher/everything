use everything_structures::{Object, Structure};

use crate::{
    inference::{Knowledge, query_values},
    objects::{self, NodeType, ObjectExt},
};

pub fn call(knowledge: &Structure, function: &Object, argument: &Object) -> Object {
    todo!("impl dynamic of {function:?} {argument:?} = ?")
}

pub fn eval(knowledge: &Structure, node: &Object) -> Object {
    match node.node_type(knowledge) {
        Some(NodeType::Call) => {
            let query = query_values(knowledge, node, &objects::NODE_CALL);
            let x = query.iter().next().unwrap();
        }
        Some(_) => todo!(),
        None => node.clone(),
    }
}
