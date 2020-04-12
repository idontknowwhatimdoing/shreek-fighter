use crate::game::{Orientation, Player, Shrek};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct ChangeShrekOrientation;

impl<'a> System<'a> for ChangeShrekOrientation {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Player>,
		ReadStorage<'a, Shrek>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, mut player, shrek, input): Self::SystemData) {
		for (sprite, player, _shrek) in (&mut sprite, &mut player, &shrek).join() {
			if let Some(mvt) = input.axis_value("shrek") {
				if mvt > 0.0 && player.orientation == Orientation::Left {
					player.orientation = Orientation::Right;
					sprite.sprite_number = 10;
				} else if mvt < 0.0 && player.orientation == Orientation::Right {
					player.orientation = Orientation::Left;
					sprite.sprite_number = 17;
				}
			}
		}
	}
}
