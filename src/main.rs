use bevy::{asset::AssetPlugin, input::common_conditions::input_toggle_active, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteBundle};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
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
        .add_startup_system(setup)
        .insert_resource(Msaa::Off)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PixelCameraBundle::from_resolution(128, 128));

    let player_sprite: Handle<Aseprite> = asset_server.load(sprites::Player::PATH);
    commands.spawn(Player).insert(AsepriteBundle {
        aseprite: player_sprite,
        animation: AsepriteAnimation::new(player_sprite.info()),
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
