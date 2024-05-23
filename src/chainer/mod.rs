//! Chaining APIs for common Rust types

mod into_chainer;
pub use into_chainer::IntoChainer;
mod with_vars;

mod slice_box;
pub use slice_box::SliceBoxChain;
mod slice_ref;
pub use slice_ref::SliceRefChain;
mod slice_mut;
pub use slice_mut::SliceMutChain;

mod array;
pub use array::ArrayChain;
mod array_ref;
pub use array_ref::ArrayRefChain;
mod array_mut;
pub use array_mut::ArrayMutChain;

mod vec;
pub use vec::VecChain;
mod vec_ref;
pub use vec_ref::VecRefChain;
mod vec_mut;
pub use vec_mut::VecMutChain;

mod string;
pub use string::StringChain;
mod string_mut;
pub use string_mut::StringMutChain;
mod string_ref;
pub use string_ref::StringRefChain;
mod str_box;
pub use str_box::StrBoxChain;
mod str_ref;
pub use str_ref::StrRefChain;
mod str_mut;
pub use str_mut::StrMutChain;

mod vec_deque;
pub use vec_deque::VecDequeChain;
mod linked_list;
pub use linked_list::LinkedListChain;

mod hash_map;
pub use hash_map::HashMapChain;
mod hash_set;
pub use hash_set::HashSetChain;

mod btree_map;
use btree_map::BTreeMapChain;
mod btree_set;
use btree_set::BTreeSetChain;

mod binary_heap;
use binary_heap::BinaryHeapChain;

#[cfg(feature = "bitstream-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitstream-unstable")))]
mod bitstream;
#[cfg(feature = "bitstream-unstable")]
pub use bitstream::BitstreamEncoderChain;
