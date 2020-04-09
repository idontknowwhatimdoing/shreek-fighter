use crate::game::{Shrek, GROUND_HEIGHT, JUMP_HEIGHT};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};

#[derive(SystemDesc, Default)]
pub struct PlayerJump {
	reached_top: bool,
	is_descending: bool,
}

impl<'a> System<'a> for PlayerJump {
	type SystemData = (
		ReadStorage<'a, Shrek>,
		WriteStorage<'a, Transform>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (shrek, mut pos, input): Self::SystemData) {
		for (_shrek, pos) in (&shrek, &mut pos).join() {
			if pos.translation().y == JUMP_HEIGHT {
				self.reached_top = true;
			} else if pos.translation().y == GROUND_HEIGHT {
				self.reached_top = false;
				self.is_descending = false;
			}

			if input.key_is_down(VirtualKeyCode::Up) && !self.reached_top && !self.is_descending {
				if pos.translation().y < JUMP_HEIGHT {
					pos.prepend_translation_y(5.0);
				}
			} else {
				let mut descending_speed = -5.0;
				if pos.translation().y > GROUND_HEIGHT {
					if input.key_is_down(VirtualKeyCode::Down) {
						descending_speed *= 2.0;
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
