pub struct Uninit {
	__private: ()
}

pub struct Init {
	__private: ()
}

/// More rudimentary builder generator helper
// I think we'd need proc macro to support generic structs properly
// (and to get rid of incrementer param)
#[macro_export]
macro_rules! generate_builder {
	{
		#[builder = $builder:ident]
		$vis:vis struct $struct_name:ident {
			$($incrementer:literal $field:ident: $type:ty),*
		}
	} => {
		$vis struct $struct_name {
			$($field: $type),*
		}

		#[allow(
			non_camel_case_types,
			clippy::allow_attributes,
			reason = "macro impl"
		)]
		#[repr(transparent)]
		$vis struct $builder<$($field = $crate::builder::Uninit),*> {
			inner: ($($crate::prelude_std::MaybeUninit<$type>,)*),
			__marker: $crate::prelude_std::PhantomData<
				// invariant? ?????
				fn(($($field,)*)) -> ($($field,)*)
			>
		}
	}
}
pub use generate_builder;

// generate_builder! {
// 	#[builder = TestBuilder]
// 	pub struct Test {
// 		0 eeie: u8,
// 		1 field: crate::prelude_std::Vec<u8>
// 	}
// }
