#[allow(non_snake_case)]
mod TextStructureProperties {
    use super::super::*;

    #[test]
    fn next_is_sorted() {
        assert!(TextStructureProperties::TailAndItem("ello, world!", 'h').is_sorted());
    }

    #[test]
    fn next() {
        let mut properties = TextStructureProperties::TailAndItem("tail", 'a');
        assert_eq!(
            properties.next(),
            Some(Property {
                tag: Object::Abstract(Abstract::LIST_ITEM),
                value: Object::Structure(Structure::Character('a'))
            })
        );
        assert_eq!(
            properties.next(),
            Some(Property {
                tag: Object::Abstract(Abstract::LIST_TAIL),
                value: Object::Structure(Structure::Text(TextStructure::new("tail").unwrap()))
            })
        );
        assert_eq!(properties.next(), None);
        assert_eq!(properties.next(), None);
        assert_eq!(properties.next(), None);
    }
}
