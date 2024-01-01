use bevy::prelude::*;
use colored::*;

use crate::score::resources::*;
use crate::GameOver;

pub fn update_score(
    score: Res<Score>
) {
    if score.is_changed() {
        println!("{} {}", "new Score:".blue() , score.value.to_string().red());
    }
}

pub fn update_highscores(
    mut game_over_event: EventReader<GameOver>,
    mut highscores: ResMut<HighScores>,
    score: Res<Score>
) {
    for _ in game_over_event.read() {
        highscores.scores.push(("Player1".to_string(), score.value))
    }
}

pub fn handle_highscore_updates(
    highscores: Res<HighScores>
) {
    if highscores.is_changed() {
        println!("{} {:?}", "High Scores:".red(), highscores.scores);
    }
}