use crate::hex::encode_hex;
use super::*;
use std::fmt::{ self, Write };

pub(super) fn render(script: &ctx::script::Script) -> Result<String, fmt::Error> {
	// TODO: do some really rough estimation of preallocate memory?
	// or maybe it doesn't matter because I told myself I'm not focusing too hard
	// on performance here, but I guess my brain is still just like yeah ><
	let mut out = String::new();

	let ctx::script::Script {
		vars,
		default_state,
		states,
		_st: _
	} = script;

	for (id, var) in vars {
		let ty_ident = var.get_ty().type_keyword();
		write!(out, "{ty_ident} var_{}", encode_hex(&id.to_le_bytes()))?;

		if let Some(val) = var.val() {
			val.with_value(&mut |val| {
				write!(out, " = {val}")
			})?;
		}

		writeln!(out, ";")?;
	}

	// TODO: need to detect... empty states then add a dummy thing
	if let Some(state) = default_state {
		writeln!(out, "default {{")?;
		// TODO: events
		writeln!(out, "}}")?;
	} else {
		// TODO: not sure what to do here?
	}

	for (id, state) in states {
		writeln!(out, "state state_{} {{", encode_hex(&id.to_le_bytes()))?;
		// TODO: do something here
		writeln!(out, "}}")?;
	}

	Ok(out)
}
