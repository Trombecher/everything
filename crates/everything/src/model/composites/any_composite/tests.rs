use std::cmp::Ordering;

use crate::model::{Atom, Composite, CompositeApi, Construction, Object};

use super::*;

fn atom(value: u128) -> Object {
    Object::Atom(Atom(value))
}

fn objects() -> [Object; 3] {
    [atom(10), atom(20), atom(30)]
}

#[test]
fn empty_composite_has_no_members() {
    let composite = AnyComposite::from_sorted([]);

    assert_eq!(composite.size(), 0);
    assert_eq!(composite.as_slice(), []);
    assert!(!composite.has(&atom(10)));
    assert_eq!(composite.properties().collect::<Vec<_>>(), []);
}

#[test]
fn exposes_the_sorted_objects() {
    let objects = objects();
    let composite = AnyComposite::from_sorted(objects.clone());

    assert_eq!(composite.size(), objects.len() as u64);
    assert_eq!(composite.as_slice(), objects.as_slice());
    for object in &objects {
        assert!(composite.has(object), "{object:?} is missing");
    }
}

#[test]
fn has_rejects_absent_objects() {
    let composite = AnyComposite::from_sorted(objects());

    assert!(!composite.has(&atom(0)));
    assert!(!composite.has(&atom(15)));
    assert!(!composite.has(&atom(40)));
    assert!(
        !composite.has(&Object::Construction(crate::model::Construction::new(
            Atom(10).into(),
            Atom(20).into(),
        )))
    );
}

#[test]
fn iteration_yields_all_objects_in_order() {
    let objects = objects();
    let mut properties = AnyComposite::from_sorted(objects.clone()).into_iter();

    for expected in objects {
        assert_eq!(properties.next(), Some(expected));
    }

    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
}

#[test]
fn debug_fmt_renders_a_composite() {
    let objects = objects();
    let composite = AnyComposite::from_sorted(objects.clone());

    assert_eq!(
        format!("{composite:?}"),
        format!(
            "⟨{}, {}, {}⟩",
            format!("{:?}", objects[0]),
            format!("{:?}", objects[1]),
            format!("{:?}", objects[2])
        )
    );
}

#[test]
fn debug_fmt_pretty_prints_one_entry_per_line() {
    let composite = AnyComposite::from_sorted(objects());

    assert_eq!(
        format!("{composite:#?}"),
        "⟨\n    @10,\n    @20,\n    @30,\n⟩"
    );
}

#[test]
fn debug_fmt_pretty_prints_nested_composites_with_indentation() {
    let construction = Construction::new(
        Atom(10).into(),
        Object::Composite(Composite::from([Atom(20).into(), Atom(30).into()])),
    );
    let composite = AnyComposite::from_sorted([Object::Construction(construction)]);

    assert_eq!(
        format!("{composite:#?}"),
        "⟨\n    @10 ·· ⟨\n        @20,\n        @30,\n    ⟩,\n⟩"
    );
}

#[test]
fn equality_compares_the_objects() {
    assert_eq!(
        AnyComposite::from_sorted(objects()),
        AnyComposite::from_sorted(objects())
    );

    let one = AnyComposite::from_sorted([atom(10), atom(20)]);
    let two = AnyComposite::from_sorted([atom(10), atom(30)]);

    assert_ne!(one, two);

    let clone = one.clone();
    assert_eq!(clone, one);
}

#[test]
fn ordering_is_lexicographic() {
    let empty = AnyComposite::from_sorted([]);
    let one = AnyComposite::from_sorted([atom(10)]);
    let one_two = AnyComposite::from_sorted([atom(10), atom(20)]);
    let one_three = AnyComposite::from_sorted([atom(10), atom(30)]);

    assert_eq!(empty.cmp(&empty), Ordering::Equal);
    assert_eq!(empty.cmp(&one), Ordering::Less);
    assert_eq!(one.cmp(&empty), Ordering::Greater);
    assert_eq!(one.cmp(&one_two), Ordering::Less);
    assert_eq!(one_two.cmp(&one_three), Ordering::Less);
    assert_eq!(
        AnyComposite::from_sorted([atom(10), atom(20)]).cmp(&one_two),
        Ordering::Equal
    );
    assert_eq!(one.partial_cmp(&one_two), Some(Ordering::Less));
}
