use std::panic::{ self, PanicInfo };

#[inline]
pub fn augment_panic_hook(hook: impl Fn(&PanicInfo<'_>) + Send + Sync + 'static) {
	augment_panic_hook_inner(Box::new(hook))
}

fn augment_panic_hook_inner(hook: impl Fn(&PanicInfo<'_>) + Send + Sync + 'static) {
	let old = panic::take_hook();
	panic::set_hook(Box::new(move |panic_info| {
		hook(panic_info);
		old(panic_info);
	}))
}
