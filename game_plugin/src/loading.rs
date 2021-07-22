use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<RawTextureAssets>()
            .init_resource::<TextureAssets>()
            .build(app);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/no.ogg")]
    pub no: Handle<AudioSource>,
}

#[derive(AssetCollection, Clone)]
pub struct RawTextureAssets {
    #[asset(path = "textures/eye_sheet.png")]
    pub eye_sheet: Handle<Texture>,
    #[asset(path = "textures/tongue_sheet.png")]
    pub tongue_sheet: Handle<Texture>,
    #[asset(path = "textures/frog_sheet.png")]
    pub frog_sheet: Handle<Texture>,
    #[asset(path = "textures/heart_sheet.png")]
    pub heart_sheet: Handle<Texture>,
    #[asset(path = "textures/shelf.jpg")]
    pub shelf: Handle<Texture>,
    #[asset(path = "textures/cauldron.png")]
    pub cauldron_sheet: Handle<Texture>,
    #[asset(path = "textures/green.png")]
    pub green: Handle<Texture>,
    #[asset(path = "textures/red.png")]
    pub red: Handle<Texture>,
    #[asset(path = "textures/yellow.png")]
    pub yellow: Handle<Texture>,
    #[asset(path = "textures/blue.png")]
    pub blue: Handle<Texture>,
    #[asset(path = "textures/grey.png")]
    pub grey: Handle<Texture>,
}

pub struct TextureAssets {
    pub eye: Handle<TextureAtlas>,
    pub tongue: Handle<TextureAtlas>,
    pub frog: Handle<TextureAtlas>,
    pub heart: Handle<TextureAtlas>,
    pub green: Handle<TextureAtlas>,
    pub red: Handle<TextureAtlas>,
    pub yellow: Handle<TextureAtlas>,
    pub blue: Handle<TextureAtlas>,
    pub grey: Handle<TextureAtlas>,
    pub cauldron: Handle<TextureAtlas>,
}

impl FromWorld for TextureAssets {
    fn from_world(world: &mut World) -> Self {
        let raw_textures = world.get_resource::<RawTextureAssets>().unwrap().clone();
        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        TextureAssets {
            eye: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.eye_sheet.clone(),
                Vec2::new(64., 64.),
                6,
                1,
            )),
            tongue: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.tongue_sheet.clone(),
                Vec2::new(64., 64.),
                6,
                1,
            )),
            frog: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.frog_sheet.clone(),
                Vec2::new(64., 64.),
                6,
                1,
            )),
            heart: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.heart_sheet.clone(),
                Vec2::new(64., 64.),
                6,
                1,
            )),
            green: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.green.clone(),
                Vec2::new(64., 64.),
                6,
                1,
            )),
            red: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.red.clone(),
                Vec2::new(64., 64.),
                6,
                1,
            )),
            yellow: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.yellow.clone(),
                Vec2::new(64., 64.),
                6,
                1,
            )),
            grey: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.grey.clone(),
                Vec2::new(64., 64.),
                6,
                1,
            )),
            blue: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.blue.clone(),
                Vec2::new(64., 64.),
                6,
                1,
            )),
            cauldron: texture_atlases.add(TextureAtlas::from_grid(
                raw_textures.cauldron_sheet.clone(),
                Vec2::new(192., 192.),
                6,
                1,
            )),
        }
    }
}
