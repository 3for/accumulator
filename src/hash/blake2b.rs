//! Make `blake2_rfc` conform to the hasher interface.
//! TODO: Add SHA-256 interface pending further development.
use super::GeneralHasher;
use blake2_rfc::blake2b::Blake2b as Blake2b_;
use rug::integer::Order;
use rug::Integer;
use std::hash::Hasher;

// 32 bytes = 256 bits.
const HASH_LENGTH_IN_BYTES: usize = 32;

pub struct Blake2b(pub Blake2b_);

impl Default for Blake2b {
  fn default() -> Self {
    Blake2b(Blake2b_::new(HASH_LENGTH_IN_BYTES))
  }
}

impl Hasher for Blake2b {
  /// We could return a truncated hash but it's easier just to not use this fn for now.
  fn finish(&self) -> u64 {
    panic!("Don't use! Prefer finalize(self).")
  }
  fn write(&mut self, bytes: &[u8]) {
    Blake2b_::update(&mut self.0, bytes)
  }
}

impl GeneralHasher for Blake2b {
  type Output = Integer;
  fn finalize(self) -> Self::Output {
    Integer::from_digits(self.0.finalize().as_bytes(), Order::MsfBe)
  }
}
