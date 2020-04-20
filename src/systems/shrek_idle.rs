use crate::game::{Orientation, Player, Shrek};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct ShrekIdle {
	frame_count: u8,
}

impl<'a> System<'a> for ShrekIdle {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Shrek>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, shrek, input): Self::SystemData) {
		if input.axis_value("shrek").unwrap() == 0.0
			&& !input.action_is_down("shrek_punch").unwrap()
			&& !input.action_is_down("shrek_jump").unwrap()
		{
			self.frame_count += 1;
			for (sprite, player, _shrek) in (&mut sprite, &player, &shrek).join() {
				match player.orientation {
					Orientation::Left => {
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
					Orientation::Right => {
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
