#[allow(non_snake_case)]
mod TextCompositeProperties {
    use super::super::*;

    #[test]
    fn next_is_sorted() {
        assert!(
            TextCompositeProperties::TailAndItem(TextComposite::new("Hello, world!").unwrap())
                .is_sorted()
        );
    }

    #[test]
    fn next() {
        let mut properties =
            TextCompositeProperties::TailAndItem(TextComposite::new("atail").unwrap());

        assert_eq!(
            properties.next(),
            Some(Property {
                tag: Object::Abstract(Abstract::LIST_ITEM),
                value: Object::Composite(Composite::Character('a'))
            })
        );
        assert_eq!(
            properties.next(),
            Some(Property {
                tag: Object::Abstract(Abstract::LIST_TAIL),
                value: Object::Composite(Composite::Text(TextComposite::new("tail").unwrap()))
            })
        );
        assert_eq!(properties.next(), None);
        assert_eq!(properties.next(), None);
        assert_eq!(properties.next(), None);
    }
}
