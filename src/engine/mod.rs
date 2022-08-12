mod air;
mod board;
mod tile;
mod turns;

pub struct Engine {
    board: board::Board,
    air_tracker: air::AirTracker,
    turn_tracker: turns::TurnTracker,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: board::Board::new(),
            air_tracker: air::AirTracker::new(),
            turn_tracker: turns::TurnTracker::new(),
        }
    }
}
