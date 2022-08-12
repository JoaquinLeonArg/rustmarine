const MAX_AIR: u8 = 12;

pub struct AirTracker {
    current_air: u8,
}

impl AirTracker {
    pub fn new() -> Self {
        AirTracker {
            current_air: MAX_AIR,
        }
    }
    pub fn consume_air(&mut self, consumed_amount: u8) {
        if consumed_amount >= self.current_air {
            self.current_air = 0
        } else {
            self.current_air -= consumed_amount
        }
    }
    pub fn is_empty(&self) -> bool {
        self.current_air == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_air() {
        let mut air_tracker = AirTracker::new();
        // Air is full, consume 1
        air_tracker.consume_air(1);
        assert_eq!(air_tracker.current_air, MAX_AIR - 1);
    }
    #[test]
    fn test_empty() {
        let mut air_tracker = AirTracker::new();
        assert_eq!(air_tracker.is_empty(), false);
        // Air is full, consume 1
        air_tracker.consume_air(1);
        assert_eq!(air_tracker.is_empty(), false);
        // Set air to 1 and consume it
        air_tracker.current_air = 1;
        air_tracker.consume_air(1);
        assert_eq!(air_tracker.is_empty(), true);
        // Try to consume another, should still be at 0 and empty
        air_tracker.consume_air(1);
        assert_eq!(air_tracker.current_air, 0);
        assert_eq!(air_tracker.is_empty(), true);
    }
}
