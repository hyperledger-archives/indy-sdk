use std::collections::VecDeque;
use services::pool::events::PoolEvent;

pub struct Commander {
    events: VecDeque<PoolEvent>
}

impl Commander {
    pub fn new() -> Self {
        Commander {
            events: VecDeque::new(),
        }
    }

    pub fn get_next_event(&self) -> Option<PoolEvent> {
        self.events.pop_front()
    }

    //TODO: push event -- formats of what will come to us?
}

mod commander_tests {
    use super::*;

    #[test]
    pub fn commander_new_works() {
        Commander::new();
    }
}