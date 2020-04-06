use crate::game::{Shrek, Side};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{Button, InputHandler, StringBindings, VirtualKeyCode};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct Walk {
	frame_count: u8,
}

impl<'a> System<'a> for Walk {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Shrek>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, shrek, input): Self::SystemData) {
		self.frame_count += 1;
		for (sprite, shrek) in (&mut sprite, &shrek).join() {
			match shrek.orientation {
				Side::Left => {
					sprite.sprite_number = 18;
					while input.button_is_down(Button::Key(VirtualKeyCode::Left)) {
						if self.frame_count == 10 {
							if sprite.sprite_number < 25 {
								sprite.sprite_number += 1;
							} else if sprite.sprite_number == 25 {
								sprite.sprite_number = 18;
							}
							self.frame_count = 0;
						}
					}
				}
				Side::Right => {
					sprite.sprite_number = 10;
					while input.button_is_down(Button::Key(VirtualKeyCode::Right)) {
						if self.frame_count == 10 {
							if sprite.sprite_number < 17 {
								sprite.sprite_number += 1;
							} else if sprite.sprite_number == 17 {
								sprite.sprite_number = 10;
							}
							self.frame_count = 0;
						}
					}
				}
			}
		}
	}
}
