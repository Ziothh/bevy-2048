use bevy::prelude::*;

#[derive(Resource)]
pub struct FontSpec {
    pub family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .expect("AssetServer to be initialised with the DefaultPlugins");

        return FontSpec {
            family: asset_server.load("fonts/FiraSans-Bold.ttf"),
        };
    }
}
