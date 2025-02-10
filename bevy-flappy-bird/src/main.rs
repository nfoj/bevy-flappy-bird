mod game;
mod menu;
mod window;

use crate::game::GamePlugin;
use crate::menu::GamePlugin;

use window::WindowPlugin;

use bevy::app::App;
use bevy::prelude::*;

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins((MenuPlugin, GamePlugin, WindowPlugin))
        .run();
}

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Main,
    Game,
}
