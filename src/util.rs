//! vit model — terminal calendar.

#[derive(Debug, Clone)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub start: String,
    pub end: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub events: Vec<Event>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { events: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_event(&mut self, title: &str, start: &str, end: &str, description: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.events.push(Event { id: id.clone(), title: title.into(), start: start.into(), end: end.into(), description: description.into() });
        id
    }

    pub fn delete_event(&mut self, id: &str) -> bool {
        if let Some(pos) = self.events.iter().position(|e| e.id == id) {
            self.events.remove(pos);
            true
        } else { false }
    }

    pub fn list(&self) -> &[Event] { &self.events }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn event_crud() {
        let mut v = Model::new();
        let id = v.add_event("Meeting", "2024-01-01 10:00", "2024-01-01 11:00", "Discuss");
        assert!(v.delete_event(&id));
    }
}
