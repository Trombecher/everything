#[allow(non_snake_case)]
mod ByteSource {
    use alloc::format;

    use super::super::*;

    #[test]
    fn parse() {
        for byte in 0..=255_u8 {
            let formatted = format!("x{byte:02X}");
            let formatted2 = format!("x{byte:02x}");

            assert_eq!(ByteSource(&formatted).parse(), Byte(byte));
            assert_eq!(ByteSource(&formatted2).parse(), Byte(byte));
        }
    }
}
