use crate::game::{Guard, Orientation, Player};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct ChangeGuardOrientation;

impl<'a> System<'a> for ChangeGuardOrientation {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Player>,
		ReadStorage<'a, Guard>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, mut player, guard, input): Self::SystemData) {
		for (sprite, player, _guard) in (&mut sprite, &mut player, &guard).join() {
			if let Some(mvt) = input.axis_value("guard") {
				if mvt > 0.0 && player.orientation == Orientation::Left {
					player.orientation = Orientation::Right;
					sprite.sprite_number = 0;
				} else if mvt < 0.0 && player.orientation == Orientation::Right {
					player.orientation = Orientation::Left;
					sprite.sprite_number = 6;
				}
			}
		}
	}
}
