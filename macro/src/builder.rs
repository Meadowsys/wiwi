use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
// use quote::{ format_ident, quote };
use syn::{ Attribute, Expr, Item, ItemFn, ItemStruct, Meta, MetaNameValue };

pub fn builder(attr: TokenStream, item: Item) -> TokenStream2 {
	match item {
		Item::Struct(item) => { impl_struct(attr, item) }
		Item::Fn(item) => { impl_fn(attr, item) }
		_ => { unimplemented!() }
	}
}

fn impl_struct(_attr: TokenStream, item: ItemStruct) -> TokenStream2 {
	let ItemStruct {
		attrs,
		vis: _,
		struct_token: _,
		ident: _,
		generics: _,
		fields: _,
		semi_token: _
	} = &item;

	let mut attrs_to_return = Vec::with_capacity(attrs.len());

	for attr in attrs {
		let Attribute {
			pound_token: _,
			style: _,
			bracket_token: _,
			meta
		} = attr;

		match meta {
			Meta::NameValue(MetaNameValue {
				path,
				eq_token: _,
				value: _
			}) => {
				let Some(_path) = path.get_ident() else {
					attrs_to_return.push(attr);
					continue
				};
			}
			Meta::List(_meta) => {}
			Meta::Path(_) => {
				attrs_to_return.push(attr);
				continue
			}
		}
	}

	// quote! {
	// 	#item
	// }
	todo!()
}

fn impl_fn(_attr: TokenStream, _item: ItemFn) -> TokenStream2 {
	todo!()
}
