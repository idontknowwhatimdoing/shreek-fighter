use crate::game::{Orientation, Player, Shrek, GROUND_HEIGHT};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct ShrekWalk {
	frame_count: u8,
}

impl<'a> System<'a> for ShrekWalk {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Shrek>,
		ReadStorage<'a, Transform>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, shrek, pos, input): Self::SystemData) {
		for (sprite, player, _shrek, pos) in (&mut sprite, &player, &shrek, &pos).join() {
			if pos.translation().y == GROUND_HEIGHT {
				match player.orientation {
					Orientation::Left => {
						if input.axis_value("shrek").unwrap() < 0.0 {
							if sprite.sprite_number < 17 || sprite.sprite_number > 23 {
								sprite.sprite_number = 17;
							}
							self.frame_count += 1;
							if self.frame_count == 4 {
								if sprite.sprite_number < 23 {
									sprite.sprite_number += 1;
								} else if sprite.sprite_number == 23 {
									sprite.sprite_number = 17;
								}
								self.frame_count = 0;
							}
						}
					}
					Orientation::Right => {
						if input.axis_value("shrek").unwrap() > 0.0 {
							if sprite.sprite_number < 10 || sprite.sprite_number > 16 {
								sprite.sprite_number = 10;
							}
							self.frame_count += 1;
							if self.frame_count == 4 {
								if sprite.sprite_number < 16 {
									sprite.sprite_number += 1;
								} else if sprite.sprite_number == 16 {
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
}
