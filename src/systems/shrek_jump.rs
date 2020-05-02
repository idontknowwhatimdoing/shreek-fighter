use crate::game::{Player, Shrek, GROUND_HEIGHT, JUMP_HEIGHT};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};

#[derive(SystemDesc, Default)]
pub struct ShrekJump {
	reached_top: bool,
	is_descending: bool,
}

impl<'a> System<'a> for ShrekJump {
	type SystemData = (
		ReadStorage<'a, Player>,
		WriteStorage<'a, Transform>,
		ReadStorage<'a, Shrek>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (player, mut pos, shrek, input): Self::SystemData) {
		for (_player, pos, _shrek) in (&player, &mut pos, &shrek).join() {
			if pos.translation().y == JUMP_HEIGHT {
				self.reached_top = true;
			} else if pos.translation().y == GROUND_HEIGHT {
				self.reached_top = false;
				self.is_descending = false;
			}

			if let Some(pressed) = input.action_is_down("shrek_jump") {
				if pressed && !self.reached_top && !self.is_descending {
					if pos.translation().y < JUMP_HEIGHT {
						pos.prepend_translation_y(7.0);
					}
				} else {
					let mut descending_speed = -7.0;
					if pos.translation().y > GROUND_HEIGHT {
						if input.key_is_down(VirtualKeyCode::Down) {
							descending_speed *= 2.0;
						}
					}
					pos.prepend_translation_y(descending_speed);
					self.is_descending = true;
				}
			}

			if pos.translation().y < GROUND_HEIGHT {
				pos.set_translation_y(GROUND_HEIGHT);
			}
		}
	}
}
