use bevy::prelude::*;
use bevy_mod_aseprite::Aseprite;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct AsepriteHandles(Vec<Handle<Aseprite>>);

pub fn load(asset_server: Res<AssetServer>, mut aseprite_handles: Res<AsepriteHandles>) {
    const names: &[&str] = &["player.ase", "creecher.ase"];

    for name in names.iter() {
        let handle: Handle<Aseprite> = asset_server.load(format!("sprites/{}", name));
        aseprite_handles.push(handle);
    }
}
