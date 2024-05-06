use super::*;
use hashbrown::*;

pub fn script(f: impl FnOnce(&mut ScriptBuilder)) -> Script {
	let mut builder = ScriptBuilder::new();
	f(&mut builder);
	Script { builder }
}

pub struct ScriptBuilder {
	id_gen: util::IDGenerator
}

impl ScriptBuilder {
	fn new() -> Self {
		let id_gen = util::IDGenerator::new();
		Self { id_gen }
	}
}

pub struct Script {
	builder: ScriptBuilder
}

impl Script {
	// pub fn print_stdout
}
