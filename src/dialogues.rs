use std::fs;
use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::constants::Constants;

#[derive(Debug, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct Dialogue {
    pub list: Vec<DialogueSection>
}

impl Dialogue {
    pub fn next_dialogue(&mut self) -> Option<DialogueSection> {
        self.list.pop()
    }
}

#[derive(Resource)]
pub struct DialogueHandle(Handle<Dialogue>);

#[derive(Debug, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct DialogueSection {
    pub character: DialogueCharacter,
    pub list: Vec<String>,
}

impl DialogueSection {
    pub fn next_dialogue(&mut self) -> Option<String> {
        self.list.pop()
    }
}

#[derive(Debug, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub enum DialogueCharacter {
    Driver,
    Passenger
}

#[derive(Resource, Default)]
pub struct DialogueState {
    pub current: Option<Dialogue>,
    pub active: bool,
}

impl DialogueState {
    pub fn load_dialogue(&mut self, mut dialogues: ResMut<Assets<Dialogue>>, dialogue: Res<DialogueHandle>) {
        if let Some(dialogue) = dialogues.remove(dialogue.0.id()) {
            self.current = Some(dialogue);
            self.active = true;
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
                TextSection::new(
                    "character: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: constants.ui.font_size,
                        color: constants.ui.font_color,
                    },
                ),
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
    let dialogue = DialogueHandle(asset_server.load("dialogues/p1.dialogue.json"));
    commands.insert_resource(dialogue);

    let dialogue = DialogueHandle(asset_server.load("dialogues/p2.dialogue.json"));
    commands.insert_resource(dialogue);
}

pub fn handle_dialogue_ui(mut state: ResMut<DialogueState>, mut ui_q: Query<&mut Text, With<DialogueText>>) {
    for mut text in &mut ui_q {

        //text.sections[1].value = format!("{:#?}", state.current);

    }
}