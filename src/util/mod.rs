#[macro_use]
mod log;

mod signal;
mod throttle;
mod hexcolor;
mod file;

pub use signal::*;
pub use throttle::*;
pub use hexcolor::*;
pub use file::*;

pub fn fnv1a_hash<T: AsRef<[u8]>>(input: T) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET_BASIS;
    for byte in input.as_ref() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}
