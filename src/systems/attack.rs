use crate::game::{Player, Side};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct Attack;

impl<'a> System<'a> for Attack {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, input): Self::SystemData) {
		if input.key_is_down(VirtualKeyCode::A) {
			for (sprite, player) in (&mut sprite, &player).join() {
				match player.orientation {
					Side::Left => {
						sprite.sprite_number = 31;
					}
					Side::Right => {
						sprite.sprite_number = 27;
					}
				}
			}
		}
	}
}
