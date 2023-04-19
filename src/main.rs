use bevy::{asset::AssetPlugin, input::common_conditions::input_toggle_active, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteBundle};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use sprites::AsepriteHandles;

mod sprites;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .build()
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(PixelCameraPlugin)
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F11)),
        )
        .add_startup_system(sprites::load)
        .add_startup_system(setup)
        .insert_resource(Msaa::Off)
        .run();
}

fn setup(
    mut commands: Commands,
    aseprite_handles: Res<AsepriteHandles>,
    aseprites: Res<Assets<Aseprite>>,
) {
    let aseprite_handle = &aseprite_handles[0];
    let aseprite = aseprites.get(aseprite_handle).unwrap();
    let animation = AsepriteAnimation::new(aseprite.info(), "idle");

    commands.spawn(PixelCameraBundle::from_resolution(128, 128));
    commands.spawn(Player).insert(AsepriteBundle {
        texture_atlas: aseprite.atlas().clone_weak(),
        sprite: TextureAtlasSprite::new(animation.current_frame()),
        aseprite: aseprite_handle.clone_weak(),
        animation,
        ..Default::default()
    });
}

#[derive(Component)]
struct Player;

#[cfg(test)]
mod tests {
    #[test]
    fn smoke_check() {
        assert!(1 + 1 == 2)
    }
}
