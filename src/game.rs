use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, NullStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
	formats::texture::ImageFormat, Camera, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
};

pub const ARENA_WIDTH: f32 = 583.33;
pub const ARENA_HEIGHT: f32 = 350.0;
pub const SHREK_WIDTH: f32 = 34.0;
pub const SHREK_HEIGHT: f32 = 43.0;
// pub const GROUND_HEIGHT: f32 = 40.0;

pub struct Game;

impl SimpleState for Game {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		world.register::<Moveable>();

		init_camera(world);

		let shrek_spritesheet = load_shrek_spritesheet(world);
		init_shrek(world, shrek_spritesheet);
	}
}

fn init_camera(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 1.0);

	world
		.create_entity()
		.with(transform)
		.with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
		.build();
}

fn load_shrek_spritesheet(world: &World) -> Handle<SpriteSheet> {
	let loader = world.read_resource::<Loader>();
	let texture_handle = loader.load(
		"textures/shrek_walk_spritesheet.png",
		ImageFormat::default(),
		(),
		&world.read_resource::<AssetStorage<Texture>>(),
	);

	loader.load(
		"textures/shrek_walk_spritesheet.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&world.read_resource::<AssetStorage<SpriteSheet>>(),
	)
}

fn init_shrek(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(SHREK_WIDTH, SHREK_HEIGHT, 0.0);

	let sprite_render = SpriteRender {
		sprite_sheet: spritesheet_handle,
		sprite_number: 0,
	};

	world
		.create_entity()
		.with(transform)
		.with(Moveable)
		.with(sprite_render)
		.build();
}

// ---[[ COMPONENTS ]]---

#[derive(Default)]
struct Moveable;

impl Component for Moveable {
	type Storage = NullStorage<Self>;
}
