use std::panic::{ self, PanicInfo };

/// Augment the panic hook, adding a closure with your own code to
/// be run before the currently set panic hook
#[inline]
pub fn augment_panic_hook(hook: impl Fn(&PanicInfo<'_>) + Send + Sync + 'static) {
	let old = panic::take_hook();
	panic::set_hook(Box::new(move |panic_info| {
		hook(panic_info);
		old(panic_info);
	}))
}
