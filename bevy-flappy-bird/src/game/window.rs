use bevy::image::ImageSamplerDesciptor;
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub struct WindowPlugin;

pub const SCREEN_WIDTH: f32 = 288.;
pub const SCREEN_HEIGHT: f32 = 512.;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".to_string(),
                        resizable: false,
                        resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptior::nearest(),
                }),
        )
    }
}
