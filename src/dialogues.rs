use bevy::prelude::*;

use std::{collections::VecDeque, time::Duration};

use crate::constants::Constants;

#[derive(Debug, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
#[serde(transparent)]
pub struct DialogueList {
    pub list: Vec<Dialogue>,
}

impl DialogueList {
    pub fn get(&self, name: &str) -> Option<Dialogue> {
        self.list.iter().find(|d| d.name == name).cloned()
    }
}

#[derive(Debug, Clone, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct Dialogue {
    pub name: String,
    pub list: VecDeque<DialogueSection>,
    current: Option<DialogueSection>,
}

impl Dialogue {
    pub fn next_dialogue(&mut self) -> Option<DialogueContent> {
        match &mut self.current {
            Some(section) => {
                let text = section.next_dialogue();
                if text.is_none() {
                    self.current = self.list.pop_front();
                    return self.next_dialogue();
                }

                return Some(DialogueContent::new(section.character, text.unwrap()));
            }
            None => {
                self.current = self.list.pop_front();

                if self.current.is_none() {
                    return None;
                }

                return self.next_dialogue();
            }
        }
    }
}

#[derive(Debug, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct DialogueContent {
    character: DialogueCharacter,
    text: String,
}

impl DialogueContent {
    fn new(character: DialogueCharacter, text: String) -> Self {
        Self { text, character }
    }
}

#[derive(Resource)]
pub struct DialogueHandle(Handle<DialogueList>);

#[derive(Debug, Clone, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct DialogueSection {
    pub character: DialogueCharacter,
    pub list: VecDeque<String>,
}

impl DialogueSection {
    pub fn next_dialogue(&mut self) -> Option<String> {
        let text = self.list.pop_front();
        text
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub enum DialogueCharacter {
    Driver,
    Passenger,
}

#[derive(Resource, Default)]
pub struct DialogueState {
    pub timer: Timer,
    pub current: Option<Dialogue>,
    pub active: bool,
}

impl DialogueState {
    pub fn load_dialogue(
        &mut self,
        name: &str,
        mut dialogues: ResMut<Assets<DialogueList>>,
        dialogue: Res<DialogueHandle>,
    ) {
        if let Some(dialogue) = dialogues.remove(dialogue.0.id()) {
            self.current = dialogue.get(name);
            self.active = true;
            self.timer = Timer::new(Duration::from_secs(2), TimerMode::Repeating);
        }
    }
}

#[derive(Component)]
pub struct DialogueText {}

#[derive(Bundle)]
pub struct DialogueBundle {
    text: DialogueText,
    text_bundle: TextBundle,
}

impl DialogueBundle {
    pub fn new(constants: Res<Constants>, font: Handle<Font>) -> Self {
        Self {
            text_bundle: TextBundle::from_sections([
                TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: constants.ui.font_size,
                    color: constants.ui.font_color,
                }),
                TextSection::from_style(TextStyle {
                    font,
                    font_size: constants.ui.font_size,
                    color: constants.ui.font_color,
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.),
                bottom: Val::Px(constants.ui.fps_text_padding),
                ..default()
            }),
            text: DialogueText {},
        }
    }
}

pub fn setup_dialogues(mut commands: Commands, asset_server: Res<AssetServer>) {
    let p1 = DialogueHandle(asset_server.load("dialogues/passenger.dialogues.json"));
    commands.insert_resource(p1);
}

pub fn handle_dialogue_ui(
    mut ui_q: Query<&mut Text, With<DialogueText>>,
    mut state: ResMut<DialogueState>,
    time: Res<Time>,
    _dialogues: ResMut<Assets<DialogueList>>,
    _dialogue: Res<DialogueHandle>,
) {
    if state.active {
        // TODO dynamic timer
        state.timer.tick(time.delta());

        if state.timer.finished() {
            if let Some(current) = &mut state.current {
                if let Some(content) = current.next_dialogue() {
                    for mut text in &mut ui_q {
                        text.sections[0].value = format!("{:?}: ", content.character);
                        text.sections[1].value = content.text.clone();
                    }
                } else {
                    for mut text in &mut ui_q {
                        text.sections[0].value = String::from("");
                        text.sections[1].value = String::from("");
                    }
                    state.active = false;
                }
            }
        }
    }
}
