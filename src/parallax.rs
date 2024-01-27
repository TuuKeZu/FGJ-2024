use bevy::prelude::*;

use crate::constants::Constants;

pub struct ParallaxPlugin;

impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, add_layers);
        app.add_systems(PostUpdate, move_layers.after(add_layers));
    }
}

#[derive(Bundle, Default)]
pub struct ParallaxSprite {
    pub transform: TransformBundle,
    pub visibility: VisibilityBundle,
    pub sprite: Sprite,
    pub images: ParallaxImages,
}

#[derive(Component, Default)]
pub struct ParallaxImages(Vec<(String, f32)>);

impl ParallaxImages {
    pub fn new(images: Vec<(impl Into<String>, f32)>) -> Self {
        Self(images.into_iter().map(|(i, h)| (i.into(), h)).collect())
    }
}

#[derive(Bundle)]
struct ParallaxLayer {
    sprite: SpriteBundle,
    height: ParallaxHeight,
}

#[derive(Component, Reflect)]
pub struct ParallaxHeight {
    pos: Vec3,
}

pub fn add_layers(
    mut commands: Commands,
    sprites_without_layers: Query<(Entity, &ParallaxImages, &Sprite), Added<ParallaxImages>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, images, sprite) in sprites_without_layers.iter() {
        commands.entity(entity).with_children(|parent| {
            for image in &images.0 {
                parent.spawn(ParallaxLayer {
                    sprite: SpriteBundle {
                        sprite: sprite.clone(),
                        texture: asset_server.load(&image.0),
                        ..Default::default()
                    },
                    height: ParallaxHeight {
                        pos: Vec3::new(0., 0., image.1),
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
