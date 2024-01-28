use bevy::prelude::*;

use crate::{atlas_loader::Atlases, constants::TILE_PX_PER_UNIT};

pub struct ParallaxPlugin;

impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, add_layers);
        app.add_systems(PostUpdate, move_layers.after(add_layers));
    }
}

#[derive(Bundle)]
pub struct ParallaxSprite {
    pub transform: TransformBundle,
    pub visibility: VisibilityBundle,
    pub images: ParallaxImages,
}

#[derive(Component)]
pub struct ParallaxImages {
    /// Atlas name in manifest
    name: String,
    /// The fields used from the sprite are `color`, `flip_{x,y}`, `custom_size`, `anchor`
    base_sprite: Sprite,
}

impl ParallaxImages {
    pub fn new_default(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            base_sprite: Default::default(),
        }
    }
    pub fn new(name: impl Into<String>, base_sprite: Sprite) -> Self {
        Self {
            name: name.into(),
            base_sprite,
        }
    }
}

#[derive(Bundle)]
struct ParallaxLayer {
    sprite_bundle: SpriteSheetBundle,
    height: ParallaxHeight,
}

#[derive(Component, Reflect)]
pub struct ParallaxHeight {
    pos: Vec3,
}

pub fn add_layers(
    mut commands: Commands,
    added_entities_without_layers: Query<(Entity, &ParallaxImages), Added<ParallaxImages>>,
    atlases: Res<Atlases>,
) {
    for (entity, ParallaxImages { name, base_sprite }) in added_entities_without_layers.iter() {
        let Some(atlas_info) = atlases.by_name.get(name) else {
            error!("could not find atlas {name}");
            continue;
        };
        let tah = &atlas_info.atlas;
        let custom_size = base_sprite
            .custom_size
            .unwrap_or(atlas_info.size / TILE_PX_PER_UNIT);
        commands.entity(entity).with_children(|parent| {
            for (index, height) in &atlas_info.parallax {
                let sprite = TextureAtlasSprite {
                    color: base_sprite.color,
                    index: *index,
                    flip_x: base_sprite.flip_x,
                    flip_y: base_sprite.flip_y,
                    custom_size: Some(custom_size),
                    anchor: base_sprite.anchor,
                };
                parent.spawn(ParallaxLayer {
                    sprite_bundle: SpriteSheetBundle {
                        sprite,
                        texture_atlas: tah.clone(),
                        ..Default::default()
                    },
                    height: ParallaxHeight {
                        pos: Vec3::new(0., 0., *height),
                    },
                });
            }
        });
    }
}
#[allow(clippy::type_complexity)]
pub fn move_layers(
    camera: Query<Entity, With<Camera2d>>,
    mut transform_params: ParamSet<(
        (TransformHelper, Query<(&mut ParallaxHeight, &Parent)>),
        Query<(&ParallaxHeight, &mut Transform)>,
    )>,
) {
    let (transform_helper, mut height_query) = transform_params.p0();
    // Start by finding camera position
    let camera = camera.get_single().unwrap();
    let camera_pos = transform_helper
        .compute_global_transform(camera)
        .unwrap()
        .translation();
    let camera_height = camera_pos.z;

    // ... then collect offsets to ParallaxHeight
    for (mut height, parent) in height_query.iter_mut() {
        let (_parent_scale, parent_rotation, parent_translation) = transform_helper
            .compute_global_transform(parent.get())
            .unwrap()
            .to_scale_rotation_translation();
        let offset = parent_translation - camera_pos;
        // let new_pos = parent_rotation.inverse().mul_vec3(offset * height.pos.z);
        let new_pos = parent_rotation.inverse().mul_vec3(offset);
        height.pos.x = new_pos.x;
        height.pos.y = new_pos.y;
    }

    // ... and finally apply transform from ParallaxHeight
    for (height, mut transform) in transform_params.p1().iter_mut() {
        let scale_factor = camera_height / (camera_height - height.pos.z).max(0.);
        transform.translation = height.pos;
        transform.translation.x *= scale_factor - 1.;
        transform.translation.y *= scale_factor - 1.;
        transform.scale = Vec3::new(scale_factor, scale_factor, scale_factor);
    }
}
