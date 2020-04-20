use crate::game::{Guard, Orientation, Player};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct GuardIdle {
	frame_count: u8,
}

impl<'a> System<'a> for GuardIdle {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Guard>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, guard, input): Self::SystemData) {
		if input.axis_value("guard").unwrap() == 0.0
			&& !input.action_is_down("guard_punch").unwrap()
			&& !input.action_is_down("guard_jump").unwrap()
		{
			self.frame_count += 1;
			for (sprite, player, _guard) in (&mut sprite, &player, &guard).join() {
				match player.orientation {
					Orientation::Left => {
						if sprite.sprite_number < 30 || sprite.sprite_number > 33 {
							sprite.sprite_number = 30;
						}
						if self.frame_count == 10 {
							if sprite.sprite_number < 33 {
								sprite.sprite_number += 1;
							} else if sprite.sprite_number == 33 {
								sprite.sprite_number = 30;
							}
							self.frame_count = 0;
						}
					}
					Orientation::Right => {
						if sprite.sprite_number < 26 || sprite.sprite_number > 29 {
							sprite.sprite_number = 26;
						}
						if self.frame_count == 10 {
							if sprite.sprite_number < 29 {
								sprite.sprite_number += 1;
							} else if sprite.sprite_number == 29 {
								sprite.sprite_number = 26;
							}
							self.frame_count = 0;
						}
					}
				}
			}
		}
	}
}
