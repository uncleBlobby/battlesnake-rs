use axum::{Json, http::StatusCode};
use serde_json::json;

use crate::models::*;

// #[axum::debug_handler]
pub async fn get_battlesnake_details() -> Json<BattlesnakeDetails> {
    let details = BattlesnakeDetails::get();
    Json(details)
}

pub async fn game_start_handler(Json(payload): Json<GameStart>) -> StatusCode {
    println!("{:?}", payload);
    StatusCode::OK
    // format!("{} OK", payload.turn)
}

pub async fn move_handler(Json(payload): Json<MoveRequest>) -> Json<serde_json::Value> {
    let mut sm = ScoredMoves::init();
    let b = payload.get_board_ref();

    let you = payload.get_you_ref();

    //println!("avoiding own neck...");
    you.avoid_own_neck(&mut sm, b);
    //println!("done avoiding own neck...");

    //println!("avoiding walls...");
    you.avoid_walls(&mut sm, b);
    //println!("done avoiding walls...");

    //println!("avoiding any snake...");
    you.avoid_any_snake(&mut sm, b);
    //println!("done avoiding any snake...");

    //println!("choosing best move...");
    let chosen_move = you.choose_move(&sm);
    //println!("done choosing best move...");

    Json(json!({"move": chosen_move.get_direction_str()}))
}

pub async fn game_end_handler(Json(payload): Json<GameOver>) -> StatusCode {
    println!("{:?}", payload);
    StatusCode::OK
}
