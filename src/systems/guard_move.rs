use crate::game::{Guard, Player, ARENA_WIDTH, GUARD_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct GuardMove;

impl<'a> System<'a> for GuardMove {
	type SystemData = (
		WriteStorage<'a, Transform>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Guard>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut pos, player, guard, input): Self::SystemData) {
		for (pos, _player, _guard) in (&mut pos, &player, &guard).join() {
			if let Some(mvt_amount) = input.axis_value("guard") {
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
