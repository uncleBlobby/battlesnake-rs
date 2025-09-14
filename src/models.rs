use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ScoredMove {
    direction: Direction,
    score: i16,
}

impl ScoredMove {
    pub fn get_direction_str(self: &Self) -> String {
        match self.direction {
            Direction::Left => "left".to_string(),
            Direction::Right => "right".to_string(),
            Direction::Down => "down".to_string(),
            Direction::Up => "up".to_string(),
        }
    }
}

pub struct ScoredMoves {
    Left: ScoredMove,
    Right: ScoredMove,
    Down: ScoredMove,
    Up: ScoredMove,
}

impl ScoredMoves {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Coord {
    x: u16,
    y: u16,
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
    pub fn avoid_own_neck(self: &Self, sm: &mut ScoredMoves, b: &Board) {
        let head = &self.head;
        let neck = &self.body[1];

        // neck is left of head

        if head.x > 0 {
            if head.x - 1 == neck.x && head.y == neck.y {
                sm.Left.score -= 100;
            }
        }

        // neck is right of head

        if head.x + 1 < b.width {
            if head.x + 1 == neck.x && head.y == neck.y {
                sm.Right.score -= 100;
            }
        }

        // neck is up from head

        if head.y + 1 < b.height {
            if head.x == neck.x && head.y == neck.y + 1 {
                sm.Up.score -= 100;
            }
        }

        if neck.y > 0 {
            if head.x == neck.x && head.y == neck.y - 1 {
                sm.Down.score -= 100;
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

            if b.coord_has_snake(target) {
                sm.Left.score -= 100;
            }
        }

        // look down

        if head.y > 0 {
            let target: &Coord = &Coord {
                x: head.x,
                y: head.y - 1,
            };

            if b.coord_has_snake(target) {
                sm.Down.score -= 100;
            }
        }

        // look right

        if head.x + 1 < b.width {
            let target: &Coord = &Coord {
                x: head.x + 1,
                y: head.y,
            };

            if b.coord_has_snake(target) {
                sm.Right.score -= 100;
            }
        }

        // look up

        if head.y + 1 < b.height {
            let target: &Coord = &Coord {
                x: head.x,
                y: head.y + 1,
            };

            if b.coord_has_snake(target) {
                sm.Up.score -= 100;
            }
        }
    }

    pub fn avoid_walls(self: &Self, sm: &mut ScoredMoves, b: &Board) {
        let head = &self.head;

        if head.x == 0 {
            sm.Left.score -= 100;
        }

        if head.x == b.width - 1 {
            sm.Right.score -= 100;
        }

        if head.y == 0 {
            sm.Down.score -= 100;
        }

        if head.y == b.height - 1 {
            sm.Up.score -= 100;
        }
    }

    pub fn choose_move(self: &Self, sm: &ScoredMoves) -> ScoredMove {
        let mut possible_moves: Vec<&ScoredMove> = Vec::new();
        for m in sm.iter() {
            // TODO: introduce threshold for determining satisfactory move
            // right now anything negative should result in death
            if m.score >= 0 {
                possible_moves.push(m);
            }
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
