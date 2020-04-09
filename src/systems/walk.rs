use crate::game::{Player, Side};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct Walk {
	frame_count: u8,
}

impl<'a> System<'a> for Walk {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, input): Self::SystemData) {
		if !input.key_is_down(VirtualKeyCode::Up) {
			for key in input.keys_that_are_down() {
				if key == VirtualKeyCode::Left || key == VirtualKeyCode::Right {
					for (sprite, player) in (&mut sprite, &player).join() {
						match player.orientation {
							Side::Left => {
								if key == VirtualKeyCode::Left {
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
							Side::Right => {
								if key == VirtualKeyCode::Right {
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
}
