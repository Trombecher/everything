pub struct EncodedStatement([u8]);

pub enum PartiallyDecodedStatement {
    Create,
    Find,
    Associate {
        
    },
}