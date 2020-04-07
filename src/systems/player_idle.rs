use crate::game::{Shrek, Side};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct PlayerIdle {
	frame_count: u8,
}

impl<'a> System<'a> for PlayerIdle {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Shrek>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, shrek, input): Self::SystemData) {
		if !input.key_is_down(VirtualKeyCode::Left) && !input.key_is_down(VirtualKeyCode::Right) {
			self.frame_count += 1;
			for (sprite, shrek) in (&mut sprite, &shrek).join() {
				match shrek.orientation {
					Side::Left => {
						if sprite.sprite_number < 5 || sprite.sprite_number > 9 {
							sprite.sprite_number = 5;
						}
						if self.frame_count == 10 {
							if sprite.sprite_number < 9 {
								sprite.sprite_number += 1;
							} else if sprite.sprite_number == 9 {
								sprite.sprite_number = 5;
							}
							self.frame_count = 0;
						}
					}
					Side::Right => {
						if sprite.sprite_number > 4 {
							sprite.sprite_number = 0;
						}
						if self.frame_count == 10 {
							if sprite.sprite_number < 4 {
								sprite.sprite_number += 1;
							} else if sprite.sprite_number == 4 {
								sprite.sprite_number = 0;
							}
							self.frame_count = 0;
						}
					}
				}
			}
		}
	}
}
