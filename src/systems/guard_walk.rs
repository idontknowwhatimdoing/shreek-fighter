use crate::game::{Guard, Orientation, Player};
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
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, guard, input): Self::SystemData) {
		if !input.action_is_down("guard_jump").unwrap() {
			if let Some(mvt) = input.axis_value("guard") {
				for (sprite, player, _guard) in (&mut sprite, &player, &guard).join() {
					match player.orientation {
						Orientation::Left => {
							if mvt < 0.0 {
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
							if mvt > 0.0 {
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
}
