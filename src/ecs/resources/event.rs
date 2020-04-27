#[derive(Clone, Copy)]
pub enum Event {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Default)]
pub struct Events {
    pub events: Vec<Event>,
    pub last_event: Option<Event>,
}

impl Events {
    pub fn handled(&mut self, handled: Vec<usize>) {
        if handled.is_empty() { return; }
        let mut new_events: Vec<Event> = Vec::new();
        for (i, e) in self.events.iter().enumerate() {
            if !handled.contains(&i) {
                new_events.push(e.clone())
            }
        }
        self.events = new_events;
    }
}

