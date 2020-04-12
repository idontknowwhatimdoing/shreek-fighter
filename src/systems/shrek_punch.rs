use crate::game::{Orientation, Player, Shrek};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct ShrekPunch;

impl<'a> System<'a> for ShrekPunch {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Shrek>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, shrek, input): Self::SystemData) {
		if input.action_is_down("shrek_punch").unwrap() {
			for (sprite, player, _shrek) in (&mut sprite, &player, &shrek).join() {
				match player.orientation {
					Orientation::Left => {
						sprite.sprite_number = 31;
					}
					Orientation::Right => {
						sprite.sprite_number = 27;
					}
				}
			}
		}
	}
}
