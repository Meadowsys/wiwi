#[macro_export]
macro_rules! void {
	($($stuff:tt)*) => {
		{
			let _ = { $($stuff)* };
		}
	}
}
pub use void;
