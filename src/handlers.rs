use axum::{Json, http::StatusCode};
use serde_json::json;

use crate::models::*;

// #[axum::debug_handler]
pub async fn get_battlesnake_details() -> Json<BattlesnakeDetails> {
    let details = BattlesnakeDetails::get();
    Json(details)
}

pub async fn game_start_handler(Json(_payload): Json<GameStart>) -> StatusCode {
    //println!("{:?}", payload);
    StatusCode::OK
    // format!("{} OK", payload.turn)
}

pub async fn move_handler(Json(payload): Json<MoveRequest>) -> Json<serde_json::Value> {
    let mut sm = ScoredMoves::init();
    let b = payload.get_board_ref();

    let you = payload.get_you_ref();

    you.avoid_own_neck(&mut sm, b);

    you.avoid_walls(&mut sm, b);

    you.avoid_any_snake(&mut sm, b);

    //prefer direction of closest food...
    // TODO: tune the weighting of the food preference
    // TODO: tune the weight of moving  toward tail
    // TODO: use flood fill results more effectively..

    if you.is_longest_snake_on_board(b) {
        you.move_toward_tail(&mut sm, b);
    }

    if you.get_missing_health() > 50 || !you.is_longest_snake_on_board(b) {
        you.move_toward_food(&mut sm, b);
    }

    // if !you.is_longest_snake_on_board(b) {}

    you.use_flood_fill(&mut sm, b);

    //println!("choosing best move...");
    let chosen_move = you.choose_move(&sm);
    //println!("done choosing best move...");

    println!("{:?}", sm);
    println!("{:?}", chosen_move);

    Json(json!({"move": chosen_move.get_direction_str()}))
}

pub async fn game_end_handler(Json(_payload): Json<GameOver>) -> StatusCode {
    //println!("{:?}", payload);
    StatusCode::OK
}
