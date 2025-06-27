use std::mem::transmute;
use std::mem::MaybeUninit;

pub struct InlineContent {
    len: u8,
    content: [MaybeUninit<u8>; 14],
}

impl TryFrom<&[u8]> for InlineContent {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() > 14 {
            Err(())
        } else {
            let mut content = [MaybeUninit::uninit(); 14];
            (&mut content[..value.len()]).copy_from_slice(unsafe { transmute(value) });

            Ok(Self {
                len: value.len() as u8,
                content,
            })
        }
    }
}

impl AsRef<[u8]> for InlineContent {
    fn as_ref(&self) -> &[u8] {
        unsafe { transmute(&self.content[..self.len as usize]) }
    }
}

impl Clone for InlineContent {
    fn clone(&self) -> Self {
        self.as_ref().try_into().unwrap()
    }
}

impl PartialEq for InlineContent {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}