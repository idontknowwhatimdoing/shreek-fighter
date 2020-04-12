use crate::game::{Guard, Orientation, Player, GROUND_HEIGHT};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct GuardWalk {
	frame_count: u8,
}

impl<'a> System<'a> for GuardWalk {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Guard>,
		ReadStorage<'a, Transform>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, guard, pos, input): Self::SystemData) {
		for (sprite, player, _guard, pos) in (&mut sprite, &player, &guard, &pos).join() {
			if pos.translation().y == GROUND_HEIGHT {
				match player.orientation {
					Orientation::Left => {
						if input.axis_value("guard").unwrap() < 0.0 {
							if sprite.sprite_number < 6 || sprite.sprite_number > 11 {
								sprite.sprite_number = 6;
							}
							self.frame_count += 1;
							if self.frame_count == 6 {
								if sprite.sprite_number < 11 {
									sprite.sprite_number += 1;
								} else if sprite.sprite_number == 11 {
									sprite.sprite_number = 6;
								}
								self.frame_count = 0;
							}
						}
					}
					Orientation::Right => {
						if input.axis_value("guard").unwrap() > 0.0 {
							if sprite.sprite_number > 5 {
								sprite.sprite_number = 0;
							}
							self.frame_count += 1;
							if self.frame_count == 6 {
								if sprite.sprite_number < 5 {
									sprite.sprite_number += 1;
								} else if sprite.sprite_number == 5 {
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
}
