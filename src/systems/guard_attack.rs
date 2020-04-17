use crate::game::{Guard, Orientation, Player};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct GuardAttack;

impl<'a> System<'a> for GuardAttack {
	type SystemData = (
		WriteStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Guard>,
		Read<'a, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut sprite, player, guard, input): Self::SystemData) {
		if input.action_is_down("guard_attack").unwrap() {
			for (sprite, player, _guard) in (&mut sprite, &player, &guard).join() {
				match player.orientation {
					Orientation::Left => {
						sprite.sprite_number = 34;
					}
					Orientation::Right => {
						sprite.sprite_number = 25;
					}
				}
			}
		}
	}
}
