use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;

static MANIFEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/atlas_manifest.toml"
));

#[derive(Deserialize)]
struct AtlasDesc {
    path: String,
    size: Vec2,
    #[serde(default = "one_usize")]
    columns: usize,
    #[serde(default = "one_usize")]
    rows: usize,
    #[serde(default = "parallax_default")]
    parallax: Vec<Option<usize>>,
    #[serde(default = "parallax_z_default")]
    parallax_z: f32,
    #[serde(default)]
    parallax_invert: bool,
}

fn parallax_default() -> Vec<Option<usize>> {
    vec![Some(0)]
}

fn one_usize() -> usize {
    1
}
fn parallax_z_default() -> f32 {
    1.
}

pub struct AtlasInfo {
    pub size: Vec2,
    pub atlas: Handle<TextureAtlas>,
    pub parallax: Vec<(usize, f32)>,
}

#[derive(Resource)]
pub struct Atlases {
    pub by_name: HashMap<String, AtlasInfo>,
}

pub fn setup_atlases(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let atlases: HashMap<String, AtlasDesc> = match toml::from_str(MANIFEST) {
        Ok(atlases) => atlases,
        Err(e) => {
            error!("failed parsing atlas_manifest.toml: {e}");
            return;
        }
    };
    let atlases = atlases
        .into_iter()
        .map(|(k, v)| {
            let texture = server.load(v.path);
            let atlas = TextureAtlas::from_grid(texture, v.size, v.columns, v.rows, None, None);
            let total = atlas.textures.len();
            let handle = texture_atlases.add(atlas);
            let layers = v
                .parallax
                .into_iter()
                .enumerate()
                .filter_map(|(i, l)| {
                    l.map(|l| {
                        let index = if v.parallax_invert { total - l - 1 } else { l };
                        (index, i as f32 * v.parallax_z)
                    })
                })
                .collect();
            let info = AtlasInfo {
                size: v.size,
                atlas: handle,
                parallax: layers,
            };
            (k, info)
        })
        .collect();
    commands.insert_resource(Atlases { by_name: atlases });
}
