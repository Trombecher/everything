#[macro_export]
macro_rules! const_assert {
    ($e:expr) => {
        const _: () = const {
            assert!($e);
        };
    };
}
