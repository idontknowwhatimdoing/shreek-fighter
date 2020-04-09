use crate::game::{Player, Side};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct ChangeOrientation;

impl<'a> System<'a> for ChangeOrientation {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Player>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, mut player, input): Self::SystemData) {
		for (sprite, player) in (&mut sprite, &mut player).join() {
			if let Some(mvt) = input.axis_value("shrek") {
				if mvt > 0.0 && player.orientation == Side::Left {
					player.orientation = Side::Right;
					sprite.sprite_number = 10;
				} else if mvt < 0.0 && player.orientation == Side::Right {
					player.orientation = Side::Left;
					sprite.sprite_number = 17;
				}
			}
			if let Some(mvt) = input.axis_value("guard") {
				if mvt > 0.0 && player.orientation == Side::Left {
					player.orientation = Side::Right;
					sprite.sprite_number = 0;
				} else if mvt < 0.0 && player.orientation == Side::Right {
					player.orientation = Side::Left;
					sprite.sprite_number = 6;
				}
			}
		}
	}
}
