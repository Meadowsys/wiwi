use super::*;

#[derive(Clone, Copy)]
pub struct Val<T> {
	ty: T,
	id: util::Identifier
}
