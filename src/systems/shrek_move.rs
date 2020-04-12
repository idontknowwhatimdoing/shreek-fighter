use crate::game::{Player, Shrek, ARENA_WIDTH, SHREK_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct ShrekMove;

impl<'a> System<'a> for ShrekMove {
	type SystemData = (
		WriteStorage<'a, Transform>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Shrek>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut pos, player, shrek, input): Self::SystemData) {
		for (pos, _player, _shrek) in (&mut pos, &player, &shrek).join() {
			if let Some(mvt_amount) = input.axis_value("shrek") {
				let scaled_amount = 2.0 * mvt_amount;
				let shrek_x = pos.translation().x;
				pos.set_translation_x(
					(shrek_x + scaled_amount)
						.min(ARENA_WIDTH - SHREK_WIDTH / 2.0)
						.max(SHREK_WIDTH / 2.0),
				);
			}
		}
	}
}
