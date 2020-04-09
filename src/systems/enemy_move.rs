use crate::game::{Guard, ARENA_WIDTH, GUARD_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct EnemyMove;

impl<'a> System<'a> for EnemyMove {
	type SystemData = (
		WriteStorage<'a, Transform>,
		ReadStorage<'a, Guard>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut pos, shrek, input): Self::SystemData) {
		for (pos, _shrek) in (&mut pos, &shrek).join() {
			let mvt = input.axis_value("guard");
			if let Some(mvt_amount) = mvt {
				let scaled_amount = 2.0 * mvt_amount;
				let guard_x = pos.translation().x;
				pos.set_translation_x(
					(guard_x + scaled_amount)
						.min(ARENA_WIDTH - GUARD_WIDTH / 2.0)
						.max(GUARD_WIDTH / 2.0),
				);
			}
		}
	}
}
