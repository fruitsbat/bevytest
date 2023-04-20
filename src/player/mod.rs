use crate::animation;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/player.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2 { x: 32.0, y: 32.0 },
        5,
        1,
        None,
        Some(Vec2 { x: 0.0, y: 32.0 }),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = animation::AnimationIndices { frame_count: 5 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        animation_indices,
        animation::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player::default(),
    ));
}

#[derive(Component)]
struct Player {
    state: State,
}

impl Default for Player {
    fn default() -> Self {
        Self { state: State::Idle }
    }
}

enum State {
    Running,
    Idle,
}
