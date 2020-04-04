use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
	formats::texture::ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
};

pub struct Game;

impl SimpleState for Game {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let mut world = data.world;
		let shrek_spritesheet = load_shrek_spritesheet(&mut world);

		// initialize_background();
		initialize_shrek(&mut world, shrek_spritesheet);
	}
}

fn load_shrek_spritesheet(world: &World) -> Handle<SpriteSheet> {
	let loader = world.read_resource::<Loader>();
	let texture_handle = loader.load(
		"assets/textures/shrek_spritesheet.png",
		ImageFormat::default(),
		(),
		&world.read_resource::<AssetStorage<Texture>>(),
	);

	loader.load(
		"assets/textures/shrek_spritesheet.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&world.read_resource::<AssetStorage<SpriteSheet>>(),
	)
}

fn initialize_shrek(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(50.0, 200.0, 0.0);

	let sprite_render = SpriteRender {
		sprite_sheet: spritesheet_handle,
		sprite_number: 0,
	};

	world
		.create_entity()
		.with(Shrek::new(50.0, 50.0))
		.with(transform)
		.with(sprite_render)
		.build();
}

// -----------------------[[ COMPONENTS ]]-----------------------------------------------

struct Shrek {
	height: f32,
	width: f32,
}

impl Shrek {
	fn new(width: f32, height: f32) -> Self {
		Shrek { width, height }
	}
}

impl Component for Shrek {
	type Storage = DenseVecStorage<Self>;
}
