use bevy::{
    ecs::{archetype::Archetypes, component::ComponentId},
    prelude::*,
    transform::commands,
};
use bevy_rapier2d::{prelude::*, rapier::geometry::CollisionEventFlags};

pub fn get_components_for_entity<'a>(
    entity: &Entity,
    archetypes: &'a Archetypes,
) -> Option<impl Iterator<Item = ComponentId> + 'a> {
    for archetype in archetypes.iter() {
        if archetype.entities().iter().any(|e| e.entity() == *entity) {
            return Some(archetype.components());
        }
    }
    None
}
