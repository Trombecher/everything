use bitflags::bitflags;

bitflags! {
    pub struct Features: u32 {
        const CREATION = 1;
        const BIN = 2;
        const NAMING = 4;
        const INT = 8;
        const FS = 16;
        const FILE_TYPES = 32;
        const NODE_COUNT = 64;
        const IMAGES = 128;
        const FAVOURITES = 256;
        const TEMPORARY_OBJECTS = 256;
        const USERSE = 256;
        const REFERENCES = 256;
    }
}