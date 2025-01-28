use std::mem::transmute;
use std::time::{SystemTime, UNIX_EPOCH};
use rusqlite::ToSql;
use rusqlite::types::ToSqlOutput;
use crate::objects::ObjectId;

#[derive(Copy, Clone, PartialEq)]
pub enum Value<'a> {
    Object(ObjectId),
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
}

impl<'a> ToSql for Value<'a> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            Value::Object(x) => x.to_sql(),
            Value::Decimal(x) => x.to_sql(),
            Value::Integer(x) => x.to_sql(),
            Value::String(x) => x.to_sql(),
            Value::Duration(x) => x.to_sql(),
            Value::DateTime(x) => x.to_sql(),
            Value::Boolean(x) => x.to_sql(),
            Value::Character(x) => (unsafe { transmute::<_, &u32>(x) }).to_sql(),
            Value::URL(x) => x.to_sql(),
            Value::Binary(x) => x.to_sql(),
            Value::Color(x) => x.to_sql(),
            Value::Email(x) => x.to_sql(),
        }
    }
}

/// A timestamp, represented as the number of seconds since January 1st, 1970.
#[derive(Copy, Clone, PartialEq)]
pub struct DateTime(pub i64);

impl DateTime {
    pub fn now() -> DateTime {
        // TODO: maybe remove unwrap() (?)
        Self(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Duration(pub u64);

#[derive(Copy, Clone, PartialEq)]
pub struct Color<'a>(&'a str);

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
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