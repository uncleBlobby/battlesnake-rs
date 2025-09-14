use core::f32;
use std::cmp::Ordering;
use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {
    pub fn get_str(self: &Self) -> String {
        match self {
            Direction::Left => "left".to_string(),
            Direction::Right => "right".to_string(),
            Direction::Down => "down".to_string(),
            Direction::Up => "up".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ScoredMove {
    direction: Direction,
    score: i16,
}

impl ScoredMove {
    pub fn get_direction_str(self: &Self) -> String {
        return self.direction.get_str();
    }
}

#[derive(Debug)]
pub struct ScoredMoves {
    Left: ScoredMove,
    Right: ScoredMove,
    Down: ScoredMove,
    Up: ScoredMove,
}

impl ScoredMoves {
    const DEATH: i16 = -1000;

    pub fn init() -> ScoredMoves {
        let l: ScoredMove = ScoredMove {
            direction: Direction::Left,
            score: 0,
        };
        let r: ScoredMove = ScoredMove {
            direction: Direction::Right,
            score: 0,
        };
        let d: ScoredMove = ScoredMove {
            direction: Direction::Down,
            score: 0,
        };
        let u: ScoredMove = ScoredMove {
            direction: Direction::Up,
            score: 0,
        };

        return ScoredMoves {
            Left: l,
            Right: r,
            Down: d,
            Up: u,
        };
    }

    fn iter(&self) -> impl Iterator<Item = &ScoredMove> {
        [&self.Left, &self.Right, &self.Down, &self.Up].into_iter()
    }
}

impl Ord for ScoredMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for ScoredMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Debug, Deserialize, Serialize, Clone, Hash)]
pub struct Coord {
    x: u16,
    y: u16,
}

pub fn reconstruct_path(mut current: Coord, came_from: &HashMap<Coord, Coord>) -> Vec<Coord> {
    let mut path = vec![current];
    while let Some(&prev) = came_from.get(&current) {
        current = prev;
        path.push(current.clone());
    }
    path.reverse();
    return path;
}

impl Coord {
    pub fn is_in_bounds(self: &Self, b: &Board) -> bool {
        if self.x >= b.width {
            return false;
        }

        if self.y >= b.height {
            return false;
        }

        return true;
    }

    pub fn is_in_snakeBody(self: &Self, b: &Board) -> bool {
        let snakes = &b.snakes;

        // TODO: check whether snake has just eaten..
        // if snake has not just eaten, safe to assume their tail spot
        // will be empty next turn

        // should be done now?

        for snake in snakes {
            if snake.ate_last_turn() {
                // snake has just eaten -- tail space will not be safe
                for body in &snake.body {
                    if self.x == body.x && self.y == body.y {
                        return true;
                    }
                }
            } else {
                for (ind, body) in snake.body.iter().enumerate() {
                    if self.x == body.x && self.y == body.y && ind != snake.body.len() - 1 {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    // pub fn is_in_own_body(self: &Self, b: &Board) -> bool {
    //     let me = &b.
    // }

    pub fn get_direction_to(self: &Self, other: &Coord) -> Direction {
        //left

        if self.x > 0 {
            if self.x - 1 == other.x && self.y == other.y {
                return Direction::Left;
            }
        }

        // right

        if self.x + 1 == other.x && self.y == other.y {
            return Direction::Right;
        }

        // down

        if self.y > 0 {
            if self.x == other.x && self.y - 1 == other.y {
                return Direction::Down;
            }
        }

        // up

        if self.x == other.x && self.y + 1 == other.y {
            return Direction::Up;
        }

        return Direction::Left;
    }

    pub fn get_distance_to(self: &Self, other: &Coord) -> u16 {
        let distanceX = self.x as i16 - other.x as i16;
        let distanceY = self.y as i16 - other.x as i16;

        let total = distanceX.abs() + distanceY.abs();

        return total as u16;
    }

    pub fn get_neighbours(self: Self, b: &Board) -> Vec<Coord> {
        let mut nbs: Vec<Coord> = Vec::new();

        if self.x > 0 {
            nbs.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }

        if self.x + 1 < b.width {
            nbs.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
        }

        if self.y > 0 {
            nbs.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }

        if self.y + 1 < b.height {
            nbs.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
        }

        return nbs;
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for Coord {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CoordWithDistance {
    x: u16,
    y: u16,
    distance: u16,
}

impl Ord for CoordWithDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for CoordWithDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn flood_fill(b: &Board, s: Coord) -> i16 {
    let start = std::time::Instant::now();

    let mut q: Vec<Coord> = Vec::new();
    let mut avail: Vec<Coord> = Vec::new();
    // let mut iters: i16 = 0;

    q.push(s);

    while !q.is_empty() && start.elapsed().as_millis() < 150 {
        // iters += 1;
        let n = q[0];
        q.remove(0);

        if n.is_in_bounds(b) && !n.is_in_snakeBody(b) {
            if !avail.contains(&n) {
                avail.push(n);

                let good_neighbours = n.get_neighbours(b);

                for n in good_neighbours {
                    q.push(n);
                }
            }
        }
    }

    return avail.len().try_into().unwrap();
}

#[derive(Clone, Copy)]
pub struct Node {
    coord: Coord,
    g_score: f32,
    f_score: f32,
    came_from: Option<Coord>,
}

pub fn _path_is_clear(p: &Vec<Coord>, b: &Board) -> bool {
    for c in p {
        if c.is_in_snakeBody(b) {
            return false;
        }
    }

    return true;
}

pub fn a_star_path_find(start: Coord, end: Coord, b: &Board) -> Option<Vec<Coord>> {
    //-> Vec<Coord>
    let mut open_set: HashMap<Coord, Node> = HashMap::new();
    let mut closed_set: HashMap<Coord, Node> = HashMap::new();
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();

    let start_node = Node {
        coord: start.clone(),
        g_score: 0.0,
        f_score: heuristic_from_n_to_end(start, end),
        came_from: None,
    };

    open_set.insert(start, start_node);

    while !open_set.is_empty() {
        let current_coord = open_set
            .iter()
            .min_by(|a, b| {
                a.1.f_score
                    .partial_cmp(&b.1.f_score)
                    .unwrap_or(Ordering::Equal)
            })
            .map(|(c, _)| *c)
            .unwrap();

        if current_coord == end {
            return Some(reconstruct_path(current_coord, &came_from));
        }

        let current_node = open_set.remove(&current_coord).unwrap();
        closed_set.insert(current_coord, current_node);

        for neighb in current_coord.get_neighbours(b) {
            if closed_set.contains_key(&neighb) {
                continue;
            }

            let mut tentative_g = current_node.g_score + 1.0;

            for snake in &b.snakes {
                if snake.body.contains(&neighb) {
                    tentative_g += 1000.0;
                }
            }

            let neighbour_node = open_set.entry(neighb).or_insert(Node {
                coord: neighb,
                g_score: f32::INFINITY,
                f_score: f32::INFINITY,
                came_from: None,
            });

            if tentative_g < neighbour_node.g_score {
                came_from.insert(neighb, current_coord);
                neighbour_node.g_score = tentative_g;
                neighbour_node.f_score = tentative_g + heuristic_from_n_to_end(neighb, end);
            }
        }
    }

    None
}

pub fn heuristic_from_n_to_end(n: Coord, end: Coord) -> f32 {
    return n.get_distance_to(&end) as f32;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Customizations {
    color: String,
    head: String,
    tail: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Game {
    id: String,
    ruleset: Ruleset,
    map: String,
    timeout: u16,
    source: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Ruleset {
    name: String,
    version: String,
    settings: RulesetSettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RulesetSettings {
    foodSpawnChance: u16,
    minimumFood: u16,
    hazardDamagePerTurn: u16,
    royale: RoyaleRules,
    squad: SquadRules,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RoyaleRules {
    shrinkEveryNTurns: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SquadRules {
    allowBodyCollisions: bool,
    sharedElimination: bool,
    sharedHealth: bool,
    sharedLength: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Board {
    height: u16,
    width: u16,
    food: Vec<Coord>,
    hazards: Vec<Coord>,
    snakes: Vec<Battlesnake>,
}

impl Board {
    fn coord_has_snake(self: &Self, c: &Coord) -> bool {
        for s in self.snakes.clone() {
            for b in s.body {
                if c.x == b.x && c.y == b.y {
                    return true;
                }
            }
        }
        return false;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: u16,
    body: Vec<Coord>,
    latency: String,
    head: Coord,
    length: u16,
    shout: String,
    squad: String,
    customizations: Customizations,
}

impl Battlesnake {
    pub fn get_missing_health(self: &Self) -> u16 {
        return 100 - self.health;
    }

    pub fn ate_last_turn(self: &Self) -> bool {
        return self.body[self.body.len() - 1] == self.body[self.body.len() - 2];
    }

    pub fn avoid_own_neck(self: &Self, sm: &mut ScoredMoves, b: &Board) {
        let head = &self.head;
        let neck = &self.body[1];

        // neck is left of head

        if head.x > 0 {
            if head.x - 1 == neck.x && head.y == neck.y {
                sm.Left.score = ScoredMoves::DEATH;
            }
        }

        // neck is right of head

        if head.x + 1 < b.width {
            if head.x + 1 == neck.x && head.y == neck.y {
                sm.Right.score = ScoredMoves::DEATH;
            }
        }

        // neck is up from head

        if head.y + 1 < b.height {
            if head.x == neck.x && head.y == neck.y + 1 {
                sm.Up.score = ScoredMoves::DEATH;
            }
        }

        if neck.y > 0 {
            if head.x == neck.x && head.y == neck.y - 1 {
                sm.Down.score = ScoredMoves::DEATH;
            }
        }
    }

    pub fn avoid_any_snake(self: &Self, sm: &mut ScoredMoves, b: &Board) {
        let head = &self.head;

        // look left
        if head.x > 0 {
            let target: &Coord = &Coord {
                x: head.x - 1,
                y: head.y,
            };

            if target.is_in_snakeBody(b) {
                sm.Left.score = ScoredMoves::DEATH;
            }
        }

        // look down

        if head.y > 0 {
            let target: &Coord = &Coord {
                x: head.x,
                y: head.y - 1,
            };

            if target.is_in_snakeBody(b) {
                sm.Down.score = ScoredMoves::DEATH;
            }
        }

        // look right

        if head.x + 1 < b.width {
            let target: &Coord = &Coord {
                x: head.x + 1,
                y: head.y,
            };

            if target.is_in_snakeBody(b) {
                sm.Right.score = ScoredMoves::DEATH;
            }
        }

        // look up

        if head.y + 1 < b.height {
            let target: &Coord = &Coord {
                x: head.x,
                y: head.y + 1,
            };

            if target.is_in_snakeBody(b) {
                sm.Up.score = ScoredMoves::DEATH;
            }
        }
    }

    pub fn avoid_walls(self: &Self, sm: &mut ScoredMoves, b: &Board) {
        let head = &self.head;

        if head.x == 0 {
            sm.Left.score = ScoredMoves::DEATH;
        }

        if head.x == b.width - 1 {
            sm.Right.score = ScoredMoves::DEATH;
        }

        if head.y == 0 {
            sm.Down.score = ScoredMoves::DEATH;
        }

        if head.y == b.height - 1 {
            sm.Up.score = ScoredMoves::DEATH;
        }
    }

    pub fn find_closest_food(self: &Self, b: &Board) -> Option<Coord> {
        if b.food.len() == 0 {
            return None;
        } else {
            // sort all foods on board by distance from head.

            let mut distanceFoods: Vec<CoordWithDistance> = Vec::new();

            for f in b.food.clone() {
                let cwd = CoordWithDistance {
                    x: f.x,
                    y: f.y,
                    distance: f.get_distance_to(&self.head),
                };
                distanceFoods.push(cwd);
            }

            distanceFoods.sort();

            return Some(Coord {
                x: distanceFoods[0].x,
                y: distanceFoods[0].y,
            });
        }
    }

    pub fn find_path_to(self: &Self, b: &Board, target: &Coord) -> Option<Vec<Coord>> {
        let path = a_star_path_find(self.head, *target, b);

        return path;
    }

    pub fn _follow_path(self: &Self, sm: &mut ScoredMoves, path: Vec<Coord>) {
        let mut pathStart = path[0];

        if pathStart == self.head {
            pathStart = path[1];
        }

        let pathDirection = self.head.get_direction_to(&pathStart);

        if pathDirection == Direction::Left {
            sm.Left.score += 10;
        }

        if pathDirection == Direction::Right {
            sm.Right.score += 10;
        }

        if pathDirection == Direction::Up {
            sm.Up.score += 10;
        }

        if pathDirection == Direction::Down {
            sm.Down.score += 10;
        }
    }

    pub fn follow_path_with_weight(
        self: &Self,
        sm: &mut ScoredMoves,
        path: Vec<Coord>,
        // b: &Board,
        weight: i16,
    ) {
        let mut pathStart = path[0];

        if pathStart == self.head && path.len() > 1 {
            pathStart = path[1];
        }

        let pathDirection = self.head.get_direction_to(&pathStart);

        if pathDirection == Direction::Left {
            sm.Left.score += weight;
        }

        if pathDirection == Direction::Right {
            sm.Right.score += weight;
        }

        if pathDirection == Direction::Up {
            sm.Up.score += weight;
        }

        if pathDirection == Direction::Down {
            sm.Down.score += weight;
        }
    }

    pub fn move_toward_tail(self: &Self, sm: &mut ScoredMoves, b: &Board) {
        let tail = self.body[self.body.len() - 1];

        let path = self.find_path_to(b, &tail);

        if path.is_some() {
            match path {
                Some(p) => {
                    // if path_is_clear(&p, b) {
                    self.follow_path_with_weight(sm, p, 1);
                    // }
                }
                None => {}
            }
        }
    }

    pub fn move_toward_food(self: &Self, sm: &mut ScoredMoves, b: &Board) {
        let cf = self.find_closest_food(b);
        if cf.is_some() {
            match cf {
                Some(c) => {
                    let path = self.find_path_to(b, &c);

                    if path.is_some() {
                        match path {
                            Some(p) => {
                                self.follow_path_with_weight(sm, p, 100 - self.health as i16);
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }
        }
    }

    pub fn _get_distance_toward_food(self: &Self, b: &Board) -> u16 {
        let cf = self.find_closest_food(b);

        if cf.is_some() {
            match cf {
                Some(c) => {
                    let path = self.find_path_to(b, &c);

                    if path.is_some() {
                        match path {
                            Some(p) => {
                                return p.len().try_into().unwrap();
                            }
                            None => {
                                return 0;
                            }
                        }
                    } else {
                        return 0;
                    }
                }
                None => {
                    return 0;
                }
            }
        } else {
            return 0;
        }
    }

    pub fn use_flood_fill(self: &Self, sm: &mut ScoredMoves, b: &Board) {
        let head = self.head;

        //let safe_neighbours = head.get_neighbours(b);

        if head.x > 0 {
            let lv = flood_fill(
                b,
                Coord {
                    x: head.x - 1,
                    y: head.y,
                },
            );
            sm.Left.score += lv;
            println!("[ff {}]: {}", "left", lv)
        }

        if head.x + 1 < b.width {
            let rv = flood_fill(
                b,
                Coord {
                    x: head.x + 1,
                    y: head.y,
                },
            );
            sm.Right.score += rv;
            println!("[ff {}]: {}", "right", rv)
        }

        if head.y > 0 {
            let dv = flood_fill(
                b,
                Coord {
                    x: head.x,
                    y: head.y - 1,
                },
            );
            sm.Down.score += dv;
            println!("[ff {}]: {}", "down", dv)
        }

        if head.y + 1 < b.height {
            let uv = flood_fill(
                b,
                Coord {
                    x: head.x,
                    y: head.y + 1,
                },
            );
            sm.Up.score += uv;
            println!("[ff {}]: {}", "up", uv)
        }
    }

    pub fn choose_move(self: &Self, sm: &ScoredMoves) -> ScoredMove {
        let mut possible_moves: Vec<&ScoredMove> = Vec::new();
        for m in sm.iter() {
            // TODO: introduce threshold for determining satisfactory move
            // right now anything negative should result in death
            //if m.score >= 0 {
            possible_moves.push(m);
            //}
        }

        possible_moves.sort_by(|a, b| b.cmp(a));

        return ScoredMove {
            direction: possible_moves[0].direction,
            score: possible_moves[0].score,
        };
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MoveRequest {
    game: Game,
    turn: u16,
    board: Board,
    you: Battlesnake,
}

impl MoveRequest {
    pub fn get_board_ref(self: &Self) -> &Board {
        return &self.board;
    }

    pub fn get_you_ref(self: &Self) -> &Battlesnake {
        return &self.you;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameOver {
    game: Game,
    turn: u16,
    board: Board,
    you: Battlesnake,
}

// #[derive(Debug, Serialize, Clone)]
// struct MoveResponse {
//     move: String,
//     shout: String,
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameStart {
    game: Game,
    turn: u16,
    board: Board,
    you: Battlesnake,
}

#[derive(Debug, Serialize, Clone)]
pub struct BattlesnakeDetails {
    apiversion: &'static str,
    author: &'static str,
    color: &'static str,
    head: &'static str,
    tail: &'static str,
    version: &'static str,
}

impl BattlesnakeDetails {
    pub fn get() -> BattlesnakeDetails {
        let bd: BattlesnakeDetails = BattlesnakeDetails {
            apiversion: ("1"),
            author: ("uncleBlobby"),
            color: ("#123456"),
            head: ("default"),
            tail: ("default"),
            version: ("0.0.1"),
        };

        return bd;
    }
}
