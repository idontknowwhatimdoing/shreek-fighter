use crate::game::{Shrek, Side};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct Blink {
	frame_count: u8,
}

impl<'a> System<'a> for Blink {
	type SystemData = (WriteStorage<'a, SpriteRender>, ReadStorage<'a, Shrek>);

	fn run(&mut self, (mut sprite, shrek): Self::SystemData) {
		self.frame_count += 1;
		for (sprite, shrek) in (&mut sprite, &shrek).join() {
			match shrek.orientation {
				Side::Left => {
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
