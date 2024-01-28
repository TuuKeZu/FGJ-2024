use bevy::prelude::*;

use crate::{
    constants::Constants,
    trigger::{Target, TriggerBundle, TriggerType},
};

pub const MISSION_TARGETS: [Vec2; 12] = [
    Vec2::new(16_000., 12_500.),
    Vec2::new(22_500., 21_000.),
    Vec2::new(24_000., 23_400.),
    Vec2::new(24_300., 18_000.),
    Vec2::new(19_000., 14_200.),
    Vec2::new(11_450., 20_600.),
    Vec2::new(16_000., 12_500.),
    Vec2::new(22_500., 21_000.),
    Vec2::new(24_000., 23_400.),
    Vec2::new(24_300., 18_000.),
    Vec2::new(19_000., 14_200.),
    Vec2::new(11_450., 20_600.),
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

#[derive(Component)]
pub struct MissionStatusText {}

#[derive(Bundle)]
pub struct MissionStatusBundle {
    pub text: MissionStatusText,
    pub text_bundle: TextBundle,
}

impl MissionStatusBundle {
    pub fn _new(constants: Res<Constants>, font: Handle<Font>) -> Self {
        Self {
            text_bundle: TextBundle::from_sections([
                TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: constants.ui.font_size * 4.,
                    color: constants.ui.font_color,
                }),
                TextSection::from_style(TextStyle {
                    font,
                    font_size: constants.ui.font_size * 4.,
                    color: constants.ui.font_color,
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(50.),
                left: Val::Percent(25.),
                top: Val::Percent(25.),
                bottom: Val::Px(constants.ui.fps_text_padding),
                ..default()
            }),
            text: MissionStatusText {},
        }
    }
}

pub fn setup_missions(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    constants: Res<Constants>,
    state: Res<MissionState>,
) {
    state.spawn_current_target(&mut commands, &constants);
    //commands.spawn(MissionStatusBundle::new(constants));
}
