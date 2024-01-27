use std::time::Duration;

use bevy::prelude::*;
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen};
use bevy_tweening::EaseFunction;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Game,
}

impl AppState {
    pub fn splash_screen() -> SplashPlugin<AppState> {
        SplashPlugin::new(AppState::Splash, AppState::Game)
            .skipable()
            .add_screen(SplashScreen {
                brands: vec![SplashItem {
                    asset: SplashAssetType::SingleImage("splash_screen/division.png".to_string()),
                    tint: Color::WHITE,
                    width: Val::Auto,
                    height: Val::Auto,
                    ease_function: EaseFunction::QuadraticInOut.into(),
                    duration: Duration::from_secs_f32(2.),
                    is_static: false,
                }],
                wait_to_start: bevy_splash_screen::WaitScreenType::AfterEnd,
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            })
            .add_screen(SplashScreen {
                brands: vec![SplashItem {
                    asset: SplashAssetType::SingleImage("splash_screen/unfake.png".to_string()),
                    tint: Color::WHITE,
                    width: Val::Auto,
                    height: Val::Auto,
                    ease_function: EaseFunction::QuadraticInOut.into(),
                    duration: Duration::from_secs_f32(2.),
                    is_static: false,
                }],
                wait_to_start: bevy_splash_screen::WaitScreenType::AfterEnd,
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            })
    }
}
