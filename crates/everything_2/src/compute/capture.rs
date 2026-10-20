use crate::{
    compute::{EvaluationContext, LazyObject},
    knowledge::Knowledge,
    model::Object,
};

pub fn capture(
    object: &Object,
    knowledge: &Knowledge,
    additional_depth: usize,
    ctx: &EvaluationContext,
) -> LazyObject {
    todo!("capture")

    // this code does not compile but I want to test stuff

    /*
    match Node::new(object, knowledge) {
        Some(Node::Function { body }) => Object::new_node(Node::Function {
            body: capture(&body, knowledge, additional_depth + 1, ctx).into_object(),
        })
        .into(),
        Some(Node::Parameter { depth }) => {
            if let Some(offset_depth) = (depth as usize).checked_sub(additional_depth) {
                // The min additional depth is 1.
                // So when the parameter depth is 1 it will refer to
                // captured parameters at an additional depth of 1.

                // We also need to escape the parameter because it may contain
                // a node (which is already evaluated because parameters
                // are always evaluated before the function).

                LazyObject::Eager(Object::new_node(Node::Literal(
                    ctx.parameter_value(offset_depth).into_object(),
                )))
            } else {
                // This parameter refers to some inner, bound function,
                // so keep it.

                LazyObject::Eager(object.clone())
            }
        }
        _ => match self {
            Self::Composite(Composite::Any(composite)) => composite
                .as_ref()
                .iter()
                .map(|property| {
                    let value = property
                        .value
                        .capture(statements, additional_depth, ctx)
                        .into_object();

                    let result = if property.value == value {
                        Ok(())
                    } else {
                        value.is_valid(statements, false)
                    };

                    match result {
                        Ok(()) => Ok(Property {
                            tag: property.tag.clone(),
                            value,
                        }),
                        // TODO: debate box
                        Err(error) => Err((value, Box::new(error))),
                    }
                })
                .transpose_into_fallible()
                .collect::<Vec<_>>()
                .map_or_else(
                    |(o, error)| {
                        warn!("invalid object {o:?} with error {error:?}; replacing with {{}}");

                        LazyObject::Object(Composite::Empty.into())
                    },
                    |mut properties| {
                        LazyObject::Object(Self::Composite(Composite::new(&mut properties)))
                    },
                ),
            _ => LazyObject::Eager(object.clone()),
        },
    }
     */
}
