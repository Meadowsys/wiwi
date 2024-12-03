#[macro_export]
macro_rules! with_cloned {
	($($stuff:tt)*) => {
		// hide potential distracting implementation details in docs
		$crate::__with_cloned_impl! { $($stuff)* }
	}
}

/// implementation detail only, do not use
#[doc(hidden)]
#[macro_export]
macro_rules! __with_cloned_impl {
	{ _ in $($stuff:tt)* } => {
		{
			// easier "removing" of the macro, eg. in case it's like, in some
			// heavily nested code, cause agh that's pain to remove and sometimes
			// you just want to keep prototyping or whatever you're doing :p
			$($stuff)*
		}
	};

	{ mut $($thing:ident),+ in $($stuff:tt)* } => {
		{
			$(
				// we only support specifying mut for all or nothing, so this is for
				// when caller is using mut for some but not all vars need to be mut
				#[allow(unused_mut)]
				let mut $thing = ::core::clone::Clone::clone(&$thing);
			)+
			$($stuff)*
		}
	};

	{ $($thing:ident),+ in $($stuff:tt)* } => {
		{
			$(
				let $thing = ::core::clone::Clone::clone(&$thing);
			)+
			$($stuff)*
		}
	};
}

#[macro_export]
macro_rules! with_cloned_2 {
	($($stuff:tt)*) => {
		// hide potential distracting implementation details in docs
		$crate::__with_cloned_impl_2! { $($stuff)* }
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __with_cloned_impl_2 {
	{ $(,)? in $($stuff:tt)* } => {
		{
			$($stuff)*
		}
	};

	{ $(,)? _ $($rest:tt)* } => {
		{
			$crate::__with_cloned_impl_2! { $($rest)* }
		}
	};

	{ $(,)? mut &$thing:ident $($rest:tt)* } => {
		{
			let mut $thing = ::core::clone::Clone::clone($thing);
			$crate::__with_cloned_impl_2! { $($rest)* }
		}
	};

	{ $(,)? mut $thing:ident $($rest:tt)* } => {
		{
			let mut $thing = ::core::clone::Clone::clone(&$thing);
			$crate::__with_cloned_impl_2! { $($rest)* }
		}
	};

	{ $(,)? &$thing:ident $($rest:tt)* } => {
		{
			let $thing = ::core::clone::Clone::clone($thing);
			$crate::__with_cloned_impl_2! { $($rest)* }
		}
	};

	{ $(,)? $thing:ident $($rest:tt)* } => {
		{
			let $thing = ::core::clone::Clone::clone(&$thing);
			$crate::__with_cloned_impl_2! { $($rest)* }
		}
	};
}

#[macro_export]
macro_rules! with_cloned_3 {
	($($stuff:tt)*) => {
		// hide potential distracting implementation details in docs
		$crate::__with_cloned_impl_3! { $($stuff)* }
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __with_cloned_impl_3 {
	{ _ => $rest:expr } => {
		$rest
	};

	// arbitrary complex expr, with seperate ident

	{ mut $name:ident = $thing:expr => $rest:expr } => {
		{
			let mut $name = ::core::clone::Clone::clone(&$thing);
			$rest
		}
	};

	{ $name:ident = $thing:expr => $rest:expr } => {
		{
			let $name = ::core::clone::Clone::clone(&$thing);
			$rest
		}
	};

	{ mut $name:ident = $thing:expr, $($rest:tt)* } => {
		{
			let mut $name = ::core::clone::Clone::clone(&$thing);
			$crate::__with_cloned_impl_3! { $($rest)* }
		}
	};

	{ $name:ident = $thing:expr, $($rest:tt)* } => {
		{
			let $name = ::core::clone::Clone::clone(&$thing);
			$crate::__with_cloned_impl_3! { $($rest)* }
		}
	};

	// deref once

	{ mut *$thing:ident => $rest:expr } => {
		{
			let mut $thing = ::core::clone::Clone::clone($thing);
			$rest
		}
	};

	{ *$thing:ident => $rest:expr } => {
		{
			let $thing = ::core::clone::Clone::clone($thing);
			$rest
		}
	};

	{ mut *$thing:ident, $($rest:tt)* } => {
		{
			let mut $thing = ::core::clone::Clone::clone($thing);
			$crate::__with_cloned_impl_3! { $($rest)* }
		}
	};

	{ *$thing:ident, $($rest:tt)* } => {
		{
			let $thing = ::core::clone::Clone::clone($thing);
			$crate::__with_cloned_impl_3! { $($rest)* }
		}
	};

	// deref twice

	{ mut **$thing:ident => $rest:expr } => {
		{
			let mut $thing = ::core::clone::Clone::clone(*$thing);
			$rest
		}
	};

	{ **$thing:ident => $rest:expr } => {
		{
			let $thing = ::core::clone::Clone::clone(*$thing);
			$rest
		}
	};

	{ mut **$thing:ident, $($rest:tt)* } => {
		{
			let mut $thing = ::core::clone::Clone::clone(*$thing);
			$crate::__with_cloned_impl_3! { $($rest)* }
		}
	};

	{ **$thing:ident, $($rest:tt)* } => {
		{
			let $thing = ::core::clone::Clone::clone(*$thing);
			$crate::__with_cloned_impl_3! { $($rest)* }
		}
	};

	// simple ident (a la original `with_cloned!`)

	{ mut $thing:ident => $rest:expr } => {
		{
			let mut $thing = ::core::clone::Clone::clone(&$thing);
			$rest
		}
	};

	{ $thing:ident => $rest:expr } => {
		{
			let $thing = ::core::clone::Clone::clone(&$thing);
			$rest
		}
	};

	{ mut $thing:ident, $($rest:tt)* } => {
		{
			let mut $thing = ::core::clone::Clone::clone(&$thing);
			$crate::__with_cloned_impl_3! { $($rest)* }
		}
	};

	{ $thing:ident, $($rest:tt)* } => {
		{
			let $thing = ::core::clone::Clone::clone(&$thing);
			$crate::__with_cloned_impl_3! { $($rest)* }
		}
	};
}

// #[cfg(test)]
// mod tests {
// 	extern crate rand;
//
// 	use crate::prelude_std::*;
// 	use super::*;
// 	use rand::thread_rng;
// 	use rand::distributions::uniform::SampleRange;
// 	use std::sync::Mutex;
// 	use std::thread;
//
// 	#[test]
// 	fn it_works() {
// 		let thing = Arc::new(Mutex::new(5));
//
// 		let join_handles = (1..=5)
// 			// no need to use clone on everything, so the macro does work as expected
// 			.map(|i| with_cloned! { thing in
// 				thread::spawn(move || {
// 					thread::sleep(std::time::Duration::from_millis((0..1000).sample_single(&mut thread_rng())));
// 					*thing.lock().unwrap() *= i;
// 				})
// 			})
// 			.collect::<Vec<_>>();
// 		let expected_result = 5 * 1 * 2 * 3 * 4 * 5;
//
// 		// but we still run it through to make sure we get expected result still,
// 		// and nothing else weird went wrong or something
// 		join_handles.into_iter().for_each(|t| t.join().unwrap());
// 		assert_eq!(expected_result, *thing.lock().unwrap());
// 	}
// }
