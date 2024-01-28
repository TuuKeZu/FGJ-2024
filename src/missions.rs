use bevy::prelude::*;

use crate::{
    constants::Constants,
    trigger::{Target, TriggerBundle, TriggerType},
};

pub const MISSION_TARGETS: [Vec2; 10] = [
    Vec2::new(500., 500.),
    Vec2::new(-500., -500.),
    Vec2::new(500., 500.),
    Vec2::new(-500., -500.),
    Vec2::new(500., 500.),
    Vec2::new(-500., -500.),
    Vec2::new(500., 500.),
    Vec2::new(-500., -500.),
    Vec2::new(500., 500.),
    Vec2::new(-500., -500.),
];

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
    pub fn spawn_current_target(&self, commands: &mut Commands, constants: &Res<Constants>) {
        let trigger_type: TriggerType = if self.mission_active {
            TriggerType::StopMission
        } else {
            TriggerType::StartMission
        };

        commands
            .spawn(TriggerBundle::new(trigger_type, &constants))
            .insert(Target {})
            .insert(Transform {
                translation: self.current_target.unwrap().extend(0.),
                ..Default::default()
            });
    }

    pub fn next_target(
        &mut self,
        commands: &mut Commands,
        constants: &Res<Constants>,
    ) -> Option<usize> {
        let idx = self.target_idx.unwrap();
        self.target_idx = Some(idx + 1);
        self.mission_active = !self.mission_active;
        self.current_target = Some(MISSION_TARGETS[idx + 1]);

        self.spawn_current_target(commands, constants);
        Some((f32::ceil(self.target_idx.unwrap() as f32 / 2.)) as usize)
    }
}

pub fn setup_missions(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    constants: Res<Constants>,
    state: Res<MissionState>,
) {
    state.spawn_current_target(&mut commands, &constants);
}
