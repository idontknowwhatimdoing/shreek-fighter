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

	fn run(&mut self, (shrek, mut transform, input): Self::SystemData) {
		for (_shrek, transform) in (&shrek, &mut transform).join() {
			if transform.translation().y == JUMP_HEIGHT {
				self.reached_top = true;
			} else if transform.translation().y == GROUND_HEIGHT {
				self.reached_top = false;
				self.is_descending = false;
			}

			if input.key_is_down(VirtualKeyCode::Up) && !self.reached_top && !self.is_descending {
				if transform.translation().y < JUMP_HEIGHT {
					transform.prepend_translation_y(5.0);
				}
			} else {
				let mut descending_speed = -5.0;
				if transform.translation().y > GROUND_HEIGHT {
					if input.key_is_down(VirtualKeyCode::Down) {
						descending_speed *= 2.0;
					}
					transform.prepend_translation_y(descending_speed);
					self.is_descending = true;
				}
			}
			if transform.translation().y < GROUND_HEIGHT {
				transform.set_translation_y(GROUND_HEIGHT);
			}
		}
	}
}
