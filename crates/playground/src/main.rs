use everything::{
    base::BASE,
    ctx::EvaluationContext,
    ext::{ObjectExt, StructureExt},
};
use everything_structures::{Abstract, Object, Structure};
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    let f: Object = Structure::new_computed(
        Structure::new_computed(
            Structure::new_set([
                Structure::new_node_parameter(0).into(),
                Structure::new_node_parameter(1).into(),
            ])
            .into(),
        )
        .into(),
    )
    .into();

    assert_eq!(
        f.call(
            &BASE,
            &[
                Object::Abstract(Abstract(1337)),
                Object::Abstract(Abstract(1338))
            ],
            &mut EvaluationContext::default(),
        ),
        Structure::new_set([
            Object::Abstract(Abstract(1337)),
            Object::Abstract(Abstract(1338))
        ])
        .into()
    );
}
