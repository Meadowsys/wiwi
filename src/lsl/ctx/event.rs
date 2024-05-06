use super::*;

pub struct Event {
	_st: SingleThreadMarker
}

fn enter() {
	CONTEXT.with_borrow_mut(|ctx| match ctx.len() {
		0 => { panic!("cannot enter event context outside of script context") }
		1 => { panic!("cannot enter event context outside of state context") }
		3.. => { panic!("cannot enter event context while in sub context") }
		2 => {
			let _ctx = ctx.last_mut()
				.unwrap()
				.borrow_state_ctx();

			ctx.push(ctx::Ctx::Event {
				ctx: Event::new()
			})
		}
	});
}

fn exit() {
	CONTEXT.with_borrow_mut(|ctx| match ctx.len() {
		..=2 => { panic!("cannot exit non-existent event context") }
		4.. => { panic!("cannot exit event context while in sub context") }
		3 => {
			let event = ctx.pop()
				.unwrap()
				.unwrap_event_ctx();
			let state = ctx.last_mut()
				.unwrap()
				.borrow_state_ctx();
			// TODO: do something with these
		}
	})
}

impl Event {
	fn new() -> Self {
		Self {
			_st: PhantomData
		}
	}
}
