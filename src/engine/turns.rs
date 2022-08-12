use rand::{thread_rng, RngCore};
use std::os::unix::thread;

const DAY_TURNS: u8 = 12;
const NIGHT_TURNS: u8 = 12;

#[derive(Debug, PartialEq, Eq)]
pub enum TimeOfDay {
    Day,
    Night,
}

pub struct TurnTracker {
    turn_number: u8,
    starting_turn: u8,
}

impl TurnTracker {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let starting_turn = ((rng.next_u32() % 6) * 4) as u8;
        TurnTracker {
            turn_number: 0,
            starting_turn,
        }
    }
    fn get_total_turns() -> u8 {
        DAY_TURNS + NIGHT_TURNS
    }
    pub fn get_time_of_day(&self) -> TimeOfDay {
        match self.get_current_turn() {
            0..DAY_TURNS => TimeOfDay::Day,
            DAY_TURNS.. => TimeOfDay::Night,
        }
    }
    pub fn advance_turn(&mut self) {
        self.turn_number += 1;
    }
    pub fn get_current_turn(&self) -> u8 {
        (self.starting_turn + self.turn_number) % Self::get_total_turns()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_turn() {
        let mut turn_tracker = TurnTracker::new();
        // Game is 4 turns into the night, and started on turn 4
        turn_tracker.turn_number = 4;
        turn_tracker.starting_turn = DAY_TURNS;
        assert_eq!(turn_tracker.get_current_turn(), DAY_TURNS + 4);
        turn_tracker.advance_turn();
        // Game should be 5 turns into the night
        assert_eq!(turn_tracker.get_current_turn(), DAY_TURNS + 5);
    }
    #[test]
    fn test_get_time_of_day() {
        let mut turn_tracker = TurnTracker::new();
        // Game is on the last turn of day, and started on turn 4
        turn_tracker.turn_number = DAY_TURNS - 4 - 1;
        turn_tracker.starting_turn = 4;
        assert_eq!(turn_tracker.get_current_turn(), DAY_TURNS - 1);
        assert_eq!(turn_tracker.get_time_of_day(), TimeOfDay::Day);
        turn_tracker.advance_turn();
        // Should be on first turn of night now
        assert_eq!(turn_tracker.get_current_turn(), DAY_TURNS);
        assert_eq!(turn_tracker.get_time_of_day(), TimeOfDay::Night);
    }
    #[test]
    fn test_time_overflow() {
        let mut turn_tracker = TurnTracker::new();
        // Game is on the last turn of night, and started on first day of night
        turn_tracker.turn_number = NIGHT_TURNS - 1;
        turn_tracker.starting_turn = DAY_TURNS;
        assert_eq!(turn_tracker.get_current_turn(), DAY_TURNS + NIGHT_TURNS - 1);
        assert_eq!(turn_tracker.get_time_of_day(), TimeOfDay::Night);
        turn_tracker.advance_turn();
        // Should be on first turn of day now
        assert_eq!(turn_tracker.get_current_turn(), 0);
        assert_eq!(turn_tracker.get_time_of_day(), TimeOfDay::Day);
    }
}
