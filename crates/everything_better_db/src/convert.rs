/// Casts a u64 to a usize. This function is only
/// available on 64 bit targets.
#[allow(clippy::cast_possible_truncation)]
#[cfg(target_pointer_width = "64")]
pub fn safe_u64_to_usize(u: u64) -> usize {
    u as usize
}
