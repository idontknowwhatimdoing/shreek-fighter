use crate::game::{Player, ARENA_WIDTH, GUARD_WIDTH, SHREK_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct Move;

impl<'a> System<'a> for Move {
	type SystemData = (
		WriteStorage<'a, Transform>,
		ReadStorage<'a, Player>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut pos, player, input): Self::SystemData) {
		for (pos, _player) in (&mut pos, &player).join() {
			if let Some(mvt_amount) = input.axis_value("shrek") {
				let scaled_amount = 2.0 * mvt_amount;
				let shrek_x = pos.translation().x;
				pos.set_translation_x(
					(shrek_x + scaled_amount)
						.min(ARENA_WIDTH - SHREK_WIDTH / 2.0)
						.max(SHREK_WIDTH / 2.0),
				);
			}
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
