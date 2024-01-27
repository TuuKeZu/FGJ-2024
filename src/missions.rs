use bevy::prelude::*;

use crate::{
    constants::Constants,
    trigger::{TriggerBundle, TriggerType},
};

pub const MISSION_TARGETS: [Vec2; 2] = [Vec2::new(200., 200.), Vec2::new(-200., -200.)];

#[derive(Debug, Resource)]
pub struct MissionState {
    pub target_idx: Option<usize>,
    pub current_target: Option<Vec2>,
    pub mission_active: bool,
}

impl Default for MissionState {
    fn default() -> Self {
        Self {
            target_idx: Some(0),
            current_target: Some(MISSION_TARGETS[0]),
            mission_active: false,
        }
    }
}

impl MissionState {
    pub fn spawn_current_target(&self, mut commands: Commands, constants: Res<Constants>) {
        let trigger_type: TriggerType = if self.mission_active {
            TriggerType::StopMission
        } else {
            TriggerType::StartMission
        };

        commands.spawn(TriggerBundle::new(trigger_type, &constants)).insert(Transform {
            translation: self.current_target.unwrap().extend(0.),
            ..Default::default()
        });
    }

    pub fn next_target(&mut self) {}
}

pub fn setup_missions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    constants: Res<Constants>,
    state: Res<MissionState>,
) {
    state.spawn_current_target(commands, constants);
}
