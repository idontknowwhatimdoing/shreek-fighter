use crate::game::{Guard, Orientation, Player};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct GuardPunch {
	frame_count: u8,
}

impl<'a> System<'a> for GuardPunch {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Guard>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, guard, input): Self::SystemData) {
		if input.action_is_down("guard_punch").unwrap() {
			for (sprite, player, _guard) in (&mut sprite, &player, &guard).join() {
				match player.orientation {
					Orientation::Left => {
						if sprite.sprite_number < 21 || sprite.sprite_number > 25 {
							sprite.sprite_number = 21;
						}
						self.frame_count += 1;
						if self.frame_count == 8 {
							if sprite.sprite_number < 25 {
								sprite.sprite_number += 1;
							} else if sprite.sprite_number == 25 {
								sprite.sprite_number = 21;
							}
							self.frame_count = 0;
						}
					}
					Orientation::Right => {
						if sprite.sprite_number < 16 || sprite.sprite_number > 20 {
							sprite.sprite_number = 16;
						}
						self.frame_count += 1;
						if self.frame_count == 8 {
							if sprite.sprite_number < 20 {
								sprite.sprite_number += 1;
							} else if sprite.sprite_number == 20 {
								sprite.sprite_number = 16;
							}
							self.frame_count = 0;
						}
					}
				}
			}
		}
	}
}
