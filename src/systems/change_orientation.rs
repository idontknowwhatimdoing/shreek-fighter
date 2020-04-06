use crate::game::{Shrek, Side};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct ChangeOrientation;

impl<'a> System<'a> for ChangeOrientation {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		WriteStorage<'a, Shrek>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, mut shrek, input): Self::SystemData) {
		for (sprite, shrek) in (&mut sprite, &mut shrek).join() {
			if shrek.orientation == Side::Left && input.key_is_down(VirtualKeyCode::Right) {
				shrek.orientation = Side::Right;
				sprite.sprite_number = 9;
			} else if shrek.orientation == Side::Right && input.key_is_down(VirtualKeyCode::Left) {
				shrek.orientation = Side::Left;
				sprite.sprite_number = 18;
			}
		}
	}
}
