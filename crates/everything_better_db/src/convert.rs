#[allow(clippy::cast_possible_truncation)]
pub fn safe_u64_to_usize(u: u64) -> usize {
    #[cfg(not(target_pointer_width = "64"))]
    compile_error!("cannot cast a u64 to a usize");

    u as usize
}
