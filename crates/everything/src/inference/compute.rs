use everything_structures::{Object, Structure};

use crate::{
    ext::{self, NodeType, ObjectExt},
    inference::{Knowledge, query_values},
};

pub fn call(knowledge: &Structure, function: &Object, argument: &Object) -> Object {
    todo!("impl dynamic of {function:?} {argument:?} = ?")
}

pub fn eval(knowledge: &Structure, node: &Object) -> Object {
    match node.node_type(knowledge) {
        Some(NodeType::Call) => {
            // let query = query_values(knowledge, node, &objects::NODE_CALL);
            // let x = query.iter().next().unwrap();
            todo!()
        }
        Some(_) => todo!(),
        None => node.clone(),
    }
}
