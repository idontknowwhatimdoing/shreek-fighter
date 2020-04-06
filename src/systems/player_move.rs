use crate::game::{Shrek, Side, ARENA_WIDTH, SHREK_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::input::{Button, InputHandler, StringBindings, VirtualKeyCode};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct PlayerMove;

impl<'a> System<'a> for PlayerMove {
	type SystemData = (
		WriteStorage<'a, Transform>,
		WriteStorage<'a, Shrek>,
		WriteStorage<'a, SpriteRender>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut transform, mut shrek, mut sprite, input): Self::SystemData) {
		for (transform, shrek, sprite) in (&mut transform, &mut shrek, &mut sprite).join() {
			if input.button_is_down(Button::Key(VirtualKeyCode::Left)) {
				shrek.orientation = Side::Left;
				sprite.sprite_number = 5;
			}
			if input.button_is_down(Button::Key(VirtualKeyCode::Right)) {
				shrek.orientation = Side::Right;
				sprite.sprite_number = 0;
			}

			let mvt = input.axis_value("shrek");
			if let Some(mv_amount) = mvt {
				let scaled_amount = 2.0 * mv_amount as f32;
				let shrek_x = transform.translation().x;
				transform.set_translation_x(
					(shrek_x + scaled_amount)
						.min(ARENA_WIDTH - SHREK_WIDTH / 2.0)
						.max(SHREK_WIDTH / 2.0),
				);
			}
		}
	}
}
