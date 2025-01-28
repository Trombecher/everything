use byte_reader::Cursor;
use crate::proto::IncomingMessage;
use crate::values::Value;

pub trait Decode<T> {
    fn next(&mut self) -> Option<T>;
}

impl<'a, T> Decode<Option<T>> for Cursor<'a> where Cursor<'a>: Decode<T> {
    fn next(&mut self) -> Option<Option<T>> {
        match self.next() {
            Some(0) => Some(Some(<Self as Decode<T>>::next(self)?)),
            Some(1) => Some(None),
            _ => None
        }
    }
}

impl<'a> Decode<Value<'a>> for Cursor<'a> {
    fn next(&mut self) -> Option<Value<'a>> {
        match self.next() {
            Some(0) => Some(Value::Integer(self.next_i64_le()?)),
            _ => todo!(),
        }
    }
}

impl<'a> Decode<IncomingMessage<'a>> for Cursor<'a> {
    fn next(&mut self) -> Option<IncomingMessage<'a>> {
        todo!()
    }
}