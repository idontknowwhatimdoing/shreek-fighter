use crate::game::{Guard, Orientation, Player, Shrek, GUARD_HEIGHT, GUARD_WIDTH, SHREK_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct ShrekHitDetection;

impl<'a> System<'a> for ShrekHitDetection {
	type SystemData = (
		ReadStorage<'a, Transform>,
		ReadStorage<'a, SpriteRender>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Shrek>,
		ReadStorage<'a, Guard>,
	);

	fn run(&mut self, (pos, sprite, player, shrek, guard): Self::SystemData) {
		for (guard_pos, _guard) in (&pos, &guard).join() {
			let guard_x = guard_pos.translation().x;
			let guard_y = guard_pos.translation().y;

			for (shrek_pos, sprite, player, _shrek) in (&pos, &sprite, &player, &shrek).join() {
				let shrek_x = shrek_pos.translation().x;
				let shrek_y = shrek_pos.translation().y;
				match player.orientation {
					Orientation::Left => {
						if sprite.sprite_number == 29 || sprite.sprite_number == 31 {
							if shrek_x - SHREK_WIDTH <= guard_x + GUARD_WIDTH / 2.0 {
								if shrek_x - SHREK_WIDTH >= guard_x - GUARD_WIDTH / 2.0 {
									if shrek_y >= guard_y - GUARD_HEIGHT / 2.0 {
										if shrek_y <= guard_y + GUARD_HEIGHT / 2.0 {
											println!("[DEBUG] shrek hits guard");
										}
									}
								}
							}
						}
					}
					Orientation::Right => {
						if sprite.sprite_number == 25 || sprite.sprite_number == 27 {
							if shrek_x + SHREK_WIDTH >= guard_x - GUARD_WIDTH / 2.0 {
								if shrek_x + SHREK_WIDTH <= guard_x + GUARD_WIDTH / 2.0 {
									if shrek_y >= guard_y - GUARD_HEIGHT / 2.0 {
										if shrek_y <= guard_y + GUARD_HEIGHT / 2.0 {
											println!("[DEBUG] shrek hits guard");
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
