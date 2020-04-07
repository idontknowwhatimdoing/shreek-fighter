use crate::game::{Shrek, Side};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct PlayerPunch;

impl<'a> System<'a> for PlayerPunch {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Shrek>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, shrek, input): Self::SystemData) {
		if input.key_is_down(VirtualKeyCode::A) {
			for (sprite, shrek) in (&mut sprite, &shrek).join() {
				match shrek.orientation {
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
