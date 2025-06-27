use crate::objects::ObjectId;
use std::num::NonZeroU64;

macro_rules! define_objects {
    ($($id:ident = $e:expr)+) => {
        $(
            pub const $id: ObjectId = ObjectId(NonZeroU64::new($e).unwrap());
        )+
    };
}

define_objects![
    TAG = 1
    TAG_SCHEMA = 2
    TAG_PARENT = 3
    TAG_CONSTRAINT = 4
    TAG_INFERRED = 5
    TAG_UNIQUE_ID = 6
    TAG_UNIQUE_VALUE = 7
    CONSTRAINT = 8
    CONSTRAINT_OR = 9
    CONSTRAINT_AND = 10
    CONSTRAINT_OR_NOT = 11
    CONSTRAINT_AND_NOT = 12
    CONSTRAINT_TAG = 13
    CONSTRAINT_WITH_VALUE = 14
    _CO_HAS_TAG_AND_NOT_INFERRED = 15
    _CO_CONSTRAINT = 16
    _CO_CV_TAG = 17
    _CO_CV_TAG_WITH_VALUE = 18
    _CO_CV_ANDS = 19
    _CO_CV_ORS = 20
    _CO_AND_OR_AND_NOT = 21
    _CO_OR_OR_OR_NOT = 22
];
