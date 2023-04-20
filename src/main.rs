use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
pub mod animation;
mod player;
mod sprites;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).build())
        .add_plugin(PixelCameraPlugin)
        .add_plugin(animation::AnimationPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F11)),
        )
        .add_startup_system(setup)
        .insert_resource(Msaa::Off)
        .run();
}

fn setup(mut commands: Commands) {
    debug!("setting up camera...");
    commands.spawn(PixelCameraBundle::from_resolution(168, 168));
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke_check() {
        assert!(1 + 1 == 2)
    }
}
