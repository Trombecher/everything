pub struct Decoder<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> Decoder<'a> {
    #[inline]
    pub const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            index: 0,
        }
    }

    #[inline]
    pub fn decode<T: Decode>(&mut self) -> T {
        T::decode(self)
    }
}

pub trait Decode {
    fn decode(decoder: &mut Decoder) -> Self;
}

impl Decode for u64 {
    fn decode(decoder: &mut Decoder) -> Self {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&decoder.data[decoder.index..decoder.index + 8]);
        
        decoder.index += 8;
        
        Self::from_le_bytes(bytes)
    }
}