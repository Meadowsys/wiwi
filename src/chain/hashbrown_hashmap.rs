extern crate hashbrown;

use crate::prelude_std::*;
use super::{ chain_fn, ChainInner as _ };
use hashbrown::{ DefaultHashBuilder, HashMap };

super::decl_chain! {
	generics_decl: [K, V, S]
	generics_decl_struct_def: [K, V, S = DefaultHashBuilder]
	generics: [K, V, S]
	chain: HashMapChain
	inner: HashMap<K, V, S>
}

impl<K, V> HashMapChain<K, V> {
	#[inline]
	pub fn new() -> Self {
		HashMap::new().into_chain()
	}

	#[inline]
	pub fn new_with_capacity(capacity: usize) -> Self {
		HashMap::with_capacity(capacity).into_chain()
	}
}

impl<K, V, S> HashMapChain<K, V, S> {
	#[inline]
	pub fn new_with_hasher(hash_builder: S) -> Self {
		HashMap::with_hasher(hash_builder).into_chain()
	}

	#[inline]
	pub fn new_with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
		HashMap::with_capacity_and_hasher(capacity, hash_builder).into_chain()
	}
}

/*
Methods
allocation_size
allocator
capacity
clear
contains_key
drain
entry
entry_ref
extract_if
get
get_key_value
get_key_value_mut
get_many_key_value_mut
get_many_key_value_unchecked_mut
get_many_mut
get_many_unchecked_mut
get_mut
hasher
insert
insert_unique_unchecked
into_keys
into_values
is_empty
iter
iter_mut
keys
len
new
new_in
par_drain
par_eq
par_keys
par_values
par_values_mut
raw_entry
raw_entry_mut
remove
remove_entry
reserve
retain
shrink_to
shrink_to_fit
try_insert
try_reserve
values
values_mut
Trait Implementations
Clone
Debug
Default
Deserialize<'de>
Eq
Extend<&'a (K, V)>
Extend<(&'a K, &'a V)>
Extend<(K, V)>
From<HashMap<T, (), S, A>>
From<[(K, V); N]>
FromIterator<(K, V)>
FromParallelIterator<(K, V)>
Index<&Q>
IntoIterator
IntoIterator
IntoIterator
IntoParallelIterator
IntoParallelIterator
IntoParallelIterator
ParallelExtend<(&'a K, &'a V)>
ParallelExtend<(K, V)>
PartialEq
Serialize
Auto Trait Implementations
Freeze
RefUnwindSafe
Send
Sync
Unpin
UnwindSafe
Blanket Implementations
Any
Borrow<T>
BorrowMut<T>
CloneToUninit
DeserializeOwned
Equivalent<K>
From<T>
Into<U>
IntoParallelRefIterator<'data>
IntoParallelRefMutIterator<'data>
Pointable
ToOwned
TryFrom<U>
TryInto<U>
*/
