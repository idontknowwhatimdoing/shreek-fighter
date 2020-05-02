use crate::game::{Guard, Orientation, Player, Shrek, GUARD_WIDTH, SHREK_HEIGHT, SHREK_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct GuardHitDetection;

impl<'a> System<'a> for GuardHitDetection {
	type SystemData = (
		ReadStorage<'a, Transform>,
		ReadStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Shrek>,
		ReadStorage<'a, Guard>,
	);

	fn run(&mut self, (pos, sprite, player, shrek, guard): Self::SystemData) {
		for (shrek_pos, _shrek) in (&pos, &shrek).join() {
			let shrek_x = shrek_pos.translation().x;
			let shrek_y = shrek_pos.translation().y;

			for (guard_pos, sprite, player, _guard) in (&pos, &sprite, &player, &guard).join() {
				let guard_x = guard_pos.translation().x;
				let guard_y = guard_pos.translation().y;
				match player.orientation {
					Orientation::Left => {
						if sprite.sprite_number == 23
							|| sprite.sprite_number == 24
							|| sprite.sprite_number == 25
						{
							if guard_x - GUARD_WIDTH <= shrek_x + SHREK_WIDTH {
								if guard_x - GUARD_WIDTH >= shrek_x - SHREK_WIDTH {
									if guard_y >= shrek_y - SHREK_HEIGHT / 2.0 {
										if guard_y <= shrek_y + SHREK_HEIGHT / 2.0 {
											println!("[DEBUG] guard hits shrek");
										}
									}
								}
							}
						}
					}
					Orientation::Right => {
						if sprite.sprite_number == 18
							|| sprite.sprite_number == 19
							|| sprite.sprite_number == 20
						{
							if guard_x + GUARD_WIDTH >= shrek_x - SHREK_WIDTH {
								if guard_x + GUARD_WIDTH <= shrek_x + SHREK_WIDTH {
									if guard_y >= shrek_y - SHREK_HEIGHT / 2.0 {
										if guard_y <= shrek_y + SHREK_HEIGHT / 2.0 {
											println!("[DEBUG] guard hits shrek");
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
}
