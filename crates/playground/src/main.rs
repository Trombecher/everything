use everything::{EvaluationContext, base::BASE, ext::ObjectExt};
use everything_structures::Object;
use everything_structures_ff::parse_structure;

fn main() {
    let subject: Object = parse_structure("{(@3, {(@3, {(@13, {(@14, {(@4, {(@15, @9)}), (@5, {(@3, {(@20, {(@19, @9), (@19, {(@15, @9)})}), (@20, {(@14, {(@4, {(@15, @9)}), (@5, @10)})})})})})}), (@13, {(@3, {(@19, {(@10, @9)}), (@19, {(@17, {(@18, {(@4, {(@15, {(@10, @9)})}), (@5, @10)})})})})})})})}").unwrap().into();
    let computed_constraint: Object = parse_structure(
        "{(@3, {(@19, {(@10, @9)}), (@19, {(@17, {(@18, {(@4, {(@15, @9)}), (@5, @3)})})})})}",
    )
    .unwrap()
    .into();

    let result = computed_constraint.call(&BASE, &subject, &mut EvaluationContext::default());

    println!("{result:?}");
}
