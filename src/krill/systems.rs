use bevy::prelude::*;

use crate::{assets::ImageAssets, DebugEvent};

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Krill;

#[derive(Bundle)]
pub struct KrillBundle {
    krill: Krill,
    sprite: SpriteBundle,
}

pub fn spawn_krill(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    image_assets: Res<ImageAssets>,
) {
    commands.spawn(
        KrillBundle {
            krill: Krill,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10., 10.)),
                    ..Default::default()
                },
                texture: image_assets.krill.clone(),
                transform: Transform {
                    ..Default::default()
                },
                ..Default::default()
            },
        }, // Name::new("Krill"),
    );
}
pub fn debug_krill(
    mut debug_event_reader: EventReader<DebugEvent>,
    krill_query: Query<&Transform, With<Krill>>,
) {
    for _event in debug_event_reader.read() {
        info!("{:?}", krill_query);
        let krill_transform = krill_query.get_single().unwrap();
        info!("{:?}", krill_transform);
        info!("{:?}", krill_transform.forward());
    }
}