use bevy::image::ImageSamplerDescriptor;
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub const SCREEN_WIDTH: f32 = 288.;
pub const SCREEN_HEIGHT: f32 = 512.;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".to_string(),
                        resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                }),
        )
    }
}
