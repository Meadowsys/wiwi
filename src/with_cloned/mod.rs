#[macro_export]
macro_rules! with_cloned {
	{ mut $($thing:ident),+ in $($stuff:tt)* } => {{
		$(#[allow(unused_mut)] let mut $thing = ::std::clone::Clone::clone(&$thing);)+
		$($stuff)+
	}};

	{ $($thing:ident),+ in $($stuff:tt)* } => {{
		$(#[allow(unused_mut)] let $thing = ::std::clone::Clone::clone(&$thing);)+
		$($stuff)+
	}};
}
pub use with_cloned;
