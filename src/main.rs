use bevy::prelude::*;

use crate::score::resources::*;
use crate::star::resources::*;
use crate::enemy::resources::*;

use crate::player::systems::*;
use crate::enemy::systems::*;
use crate::star::systems::*;
use crate::score::systems::*;


pub mod events;
pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use events::*;
use systems::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<HighScores>()
        .add_event::<GameOver>()
        .add_systems(Startup, (
            spawn_player,
            spawn_camera,
            spawn_enemies,
            spawn_stars
        ))
        .add_systems(Update, (
            player_movement,
            confine_player_movement,

            enemy_movement,
            confine_enemy_position,
            update_enemy_direction,

            enemy_hit_player,
            player_hit_star,

            tick_star_spawn_timer,
            spawn_stars_over_time,
            
            update_score,

            tick_enemy_spawn_timer,
            spawn_enemies_over_time,
        
            exit_game,
            handle_game_over,

            update_highscores,
            handle_highscore_updates
        ))
        .run()
}
