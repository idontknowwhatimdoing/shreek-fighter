use crate::game::{Moveable, ARENA_WIDTH, SHREK_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct PlayerMove;

impl<'a> System<'a> for PlayerMove {
	type SystemData = (
		WriteStorage<'a, Transform>,
		ReadStorage<'a, Moveable>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut transform, moveable, input): Self::SystemData) {
		for (transform, _moveable) in (&mut transform, &moveable).join() {
			let mvt = input.axis_value("shrek");

			if let Some(mv_amount) = mvt {
				let scaled_amount = 2.0 * mv_amount as f32;
				let shrek_x = transform.translation().x;
				transform.set_translation_x(
					(shrek_x + scaled_amount)
						.min(ARENA_WIDTH - SHREK_WIDTH / 2.0)
						.max(SHREK_WIDTH / 2.0),
				);
			}
		}
	}
}
