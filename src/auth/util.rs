use rand::{ Rng, thread_rng };

#[inline]
pub fn rand_array<const N: usize>() -> [u8; N] {
	let mut array = [0u8; N];
	thread_rng().fill(&mut array as &mut [u8]);
	array
}
