mod game;
mod menu;

use crate::game::GamePlugin;
use crate::game::WindowPlugin;
use crate::menu::MenuPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins((WindowPlugin, MenuPlugin, GamePlugin))
        .run();
}

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Main,
    Game,
}
