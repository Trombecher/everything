use std::sync::Arc;

use crate::{BytesStructure, structures::bytes::GLOBAL_BINARY_DATA};

#[test]
pub fn from_parts() {
    assert_eq!(BytesStructure::from_parts(&[], &[]), None);

    {
        let count = GLOBAL_BINARY_DATA.len();

        {
            let bytes = BytesStructure::from_parts(&[0, 1, 2], &[]).unwrap();
            assert_eq!(Arc::strong_count(&bytes.data), 2);
            assert_eq!(bytes.data.as_ref(), &[0, 1, 2]);

            assert_eq!(GLOBAL_BINARY_DATA.len(), count + 1);
        }

        assert_eq!(GLOBAL_BINARY_DATA.len(), count);
    }

    let bytes = BytesStructure::from_parts(&[], &[3, 4, 5]).unwrap();
    assert_eq!(bytes.data.as_ref(), &[3, 4, 5]);

    let bytes = BytesStructure::from_parts(&[0, 1, 2], &[3, 4, 5]).unwrap();
    assert_eq!(bytes.data.as_ref(), &[0, 1, 2, 3, 4, 5]);
}
