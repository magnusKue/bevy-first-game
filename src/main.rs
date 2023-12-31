use bevy::prelude::*;

mod events;
mod systems;
mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

fn main() {
    App::new()
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins((
            GamePlugin,
            MainMenuPlugin
        ))
        // Systems
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (
            exit_game,
            handle_game_over,
        ))
        .run()
}
