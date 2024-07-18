use rand::{ CryptoRng, Error, RngCore, SeedableRng };
use rand::rngs::{ OsRng, adapter::ReseedingRng };
use rand_chacha::{ ChaCha8Core, ChaCha12Core, ChaCha20Core };
use std::cell::RefCell;

// pub struct ThreadLocalChaCha8Rng;
// pub struct ThreadLocalChaCha12Rng;
// pub struct ThreadLocalChaCha20Rng;

macro_rules! decl_thread_local_rng {
	($struct_name:ident $core:ident $reseeder:ident) => {
		pub struct $struct_name;

		const _: () = {
			thread_local! {
				static RNG_STATIC: RefCell<ReseedingRng<$core, $reseeder>> = {
					let rng = $core::from_rng($reseeder).expect(concat!("could not initialise ", stringify!($core), " thread-local rng"));
					let rng = ReseedingRng::new(rng, 16384, OsRng);
					RefCell::new(rng)
				};
			}

			impl RngCore for $struct_name {
				#[inline]
				fn next_u32(&mut self) -> u32 {
					RNG_STATIC.with_borrow_mut(|rng| rng.next_u32())
				}

				#[inline]
				fn next_u64(&mut self) -> u64 {
					RNG_STATIC.with_borrow_mut(|rng| rng.next_u64())
				}

				#[inline]
				fn fill_bytes(&mut self, dest: &mut [u8]) {
					RNG_STATIC.with_borrow_mut(|rng| rng.fill_bytes(dest))
				}

				#[inline]
				fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
					RNG_STATIC.with_borrow_mut(|rng| rng.try_fill_bytes(dest))
				}
			}
		};
	}
}

decl_thread_local_rng! { ThreadLocalChaCha8Rng ChaCha8Core OsRng }
decl_thread_local_rng! { ThreadLocalChaCha12Rng ChaCha12Core OsRng }
decl_thread_local_rng! { ThreadLocalChaCha20Rng ChaCha20Core OsRng }

impl CryptoRng for ThreadLocalChaCha20Rng {}
