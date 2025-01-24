use std::marker::PhantomData;
use crate::error::{Error, InterfaceError};

pub struct MessageBuffer(Vec<u8>);

impl MessageBuffer {
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn reserve<T: FixedEncode<{ size_of::<T>() }>>(&mut self) -> Reserved<T> {
        let index = self.0.len();

        for _ in 0..size_of::<T>() {
            self.0.push(0);
        }

        Reserved {
            index,
            _marker: Default::default(),
        }
    }

    #[inline]
    pub fn encode(&mut self, value: &impl Encode) {
        Encode::encode(value, self);
    }

    #[inline]
    pub fn encode_reserved<T: FixedEncode<{ size_of::<T>() }>>(&mut self, reserved: Reserved<T>, value: &T) {
        value.encode_fixed(TryFrom::try_from(&mut self.0[reserved.index..reserved.index + size_of::<T>()]).unwrap());
    }
    
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Into<Vec<u8>> for MessageBuffer {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

pub trait Encode {
    fn encode(&self, rb: &mut MessageBuffer);
}

pub trait FixedEncode<const LENGTH: usize>: Encode {
    fn encode_fixed(&self, slice: &mut [u8; LENGTH]);
}

pub struct Reserved<T: FixedEncode<{ size_of::<T>() }>> {
    index: usize,
    _marker: PhantomData<T>,
}

macro_rules! impl_encode_for_number {
    ($ty:ty) => {
        impl Encode for $ty {
            fn encode(&self, rb: &mut MessageBuffer) {
                rb.0.extend_from_slice(&self.to_le_bytes());
            }
        }
        
        impl FixedEncode<{size_of::<$ty>()}> for $ty {
            fn encode_fixed(&self, slice: &mut [u8; size_of::<$ty>()]) {
                slice.copy_from_slice(&<$ty>::to_le_bytes(*self))
            }
        }
    };
}

impl_encode_for_number!(u8);
impl_encode_for_number!(u16);
impl_encode_for_number!(u32);
impl_encode_for_number!(u64);
impl_encode_for_number!(i8);
impl_encode_for_number!(i16);
impl_encode_for_number!(i32);
impl_encode_for_number!(i64);
impl_encode_for_number!(f32);
impl_encode_for_number!(f64);

impl Encode for Error {
    fn encode(&self, rb: &mut MessageBuffer) {
        match self {
            Self::Internal(_) => rb.encode(&1_u8),
            Self::Interface(InterfaceError::TypeMismatch) => rb.encode(&2_u8),
            Self::Interface(InterfaceError::CannotAssociateObjectWithInferredType) => rb.encode(&3_u8),
            Self::Interface(InterfaceError::ObjectDoesNotExist) => rb.encode(&3_u8),
            Self::Interface(InterfaceError::InvalidEventID) => rb.encode(&4_u8),
            Self::Interface(InterfaceError::InvalidProcedureID) => rb.encode(&5_u8),
            Self::Interface(InterfaceError::MalformedMessage) => rb.encode(&6_u8),
        }
    }
}

impl FixedEncode<1> for Error {
    fn encode_fixed(&self, slice: &mut [u8; 1]) {
        todo!()
    }
}

impl<T: Encode, E: Into<Error> + Copy> Encode for Result<T, E> {
    #[inline]
    fn encode(&self, rb: &mut MessageBuffer) {
        match self {
            Ok(t) => {
                rb.encode(&0_u8);
                rb.encode(t);
            }
            Err(error) => rb.encode(&<E as Into<Error>>::into(*error))
        }
    }
}