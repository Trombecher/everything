use std::mem::transmute;
use std::time::{SystemTime, UNIX_EPOCH};
use rusqlite::ToSql;
use rusqlite::types::ToSqlOutput;
use crate::objects::{GroupID, ObjectID, UserID};

pub enum GenericValue<'a> {
    ObjectID(ObjectID),
    Decimal(f64),
    Integer(i64),
    String(&'a str),
    Duration(Duration),
    DateTime(DateTime),
    Boolean(bool),
    Character(char),
    URL(URL<'a>),
    Binary(&'a [u8]),
    Color(Color<'a>),
    Email(Email<'a>),
    UserID(UserID),
    GroupID(GroupID),
}

impl<'a> ToSql for GenericValue<'a> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            GenericValue::ObjectID(x) => x.to_sql(),
            GenericValue::Decimal(x) => x.to_sql(),
            GenericValue::Integer(x) => x.to_sql(),
            GenericValue::String(x) => x.to_sql(),
            GenericValue::Duration(x) => x.to_sql(),
            GenericValue::DateTime(x) => x.to_sql(),
            GenericValue::Boolean(x) => x.to_sql(),
            GenericValue::Character(x) => (unsafe { transmute::<_, &u32>(x) }).to_sql(),
            GenericValue::URL(x) => x.to_sql(),
            GenericValue::Binary(x) => x.to_sql(),
            GenericValue::Color(x) => x.to_sql(),
            GenericValue::Email(x) => x.to_sql(),
            GenericValue::UserID(x) => x.to_sql(),
            GenericValue::GroupID(x) => x.to_sql(),
        }
    }
}

/// A timestamp, represented as the number of seconds since January 1st, 1970.
pub struct DateTime(pub i64);

impl DateTime {
    pub fn now() -> DateTime {
        // TODO: maybe remove unwrap() (?)
        Self(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64)
    }
}

pub struct Duration(pub u64);

pub struct Color<'a>(&'a str);

pub struct Email<'a>(&'a str);

impl<'a> Email<'a> {
    #[inline]
    pub const unsafe fn new_unchecked(value: &'a str) -> Self {
        Self(value)
    }

    #[inline]
    pub fn new(value: &'a str) -> Self {
        todo!("Email validation")
    }
}

pub struct URL<'a>(&'a str);

macro_rules! impl_trivial_to_sql {
    ($target:ty) => {
        impl ToSql for $target {
            fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
                self.0.to_sql()
            }
        }
    };
}

impl_trivial_to_sql!(Duration);
impl_trivial_to_sql!(Color<'_>);
impl_trivial_to_sql!(Email<'_>);
impl_trivial_to_sql!(URL<'_>);
impl_trivial_to_sql!(DateTime);