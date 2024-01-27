use bevy::prelude::*;

use crate::constants::Constants;

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
    /// Path to the atlas
    path: String,
    /// A vector with indices, height and sprites. This supports creating
    /// multiple layers with the same index to the atlas. The only fields used
    /// from the sprite are `color`, `flip_{x,y}`, `custom_size`, `anchor`
    indices_heights_sprites: Vec<(usize, f32, Sprite)>,
    /// Passed to [`TextureAtlas::from_grid`]
    tile_size: Vec2,
    columns: usize,
    rows: usize,
}

impl ParallaxImages {
    pub fn new(
        path: impl Into<String>,
        indices_heights_sprites: Vec<(usize, f32, Sprite)>,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
    ) -> Self {
        Self {
            path: path.into(),
            indices_heights_sprites,
            tile_size,
            columns,
            rows,
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
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (
        entity,
        ParallaxImages {
            path,
            indices_heights_sprites,
            tile_size,
            columns,
            rows,
        },
    ) in added_entities_without_layers.iter()
    {
        commands.entity(entity).with_children(|parent| {
            let handle = asset_server.load(path);
            let ta = TextureAtlas::from_grid(handle, *tile_size, *columns, *rows, None, None);
            let tah = texture_atlases.add(ta);
            // let tah = asset_server.get_handle(path).unwrap_or_else(|| {
            //     let handle = asset_server.load(path);
            //     let ta = TextureAtlas::from_grid(handle, *tile_size, *columns, *rows, None, None);
            //     texture_atlases.add(ta)
            // });

            for (index, height, sprite) in indices_heights_sprites {
                let sprite = TextureAtlasSprite {
                    color: sprite.color,
                    index: *index,
                    flip_x: sprite.flip_x,
                    flip_y: sprite.flip_y,
                    custom_size: sprite.custom_size,
                    anchor: sprite.anchor,
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
    constants: Res<Constants>,
) {
    let (transform_helper, mut height_query) = transform_params.p0();
    // Start by finding camera position
    let camera = camera.get_single().unwrap();
    let camera_pos = transform_helper
        .compute_global_transform(camera)
        .unwrap()
        .translation();

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
        let scale_factor = constants.camera.camera_height
            / (constants.camera.camera_height - height.pos.z).max(0.);
        transform.translation = height.pos;
        transform.translation.x *= scale_factor - 1.;
        transform.translation.y *= scale_factor - 1.;
        transform.scale = Vec3::new(scale_factor, scale_factor, scale_factor);
    }
}
