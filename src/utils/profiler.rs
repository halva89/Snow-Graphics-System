use std::collections::HashMap;
use std::time::{Instant, Duration};

pub struct Profiler {
    entries: HashMap<String, Duration>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn begin_scope(&mut self, name: &str) {
        self.entries.insert(name.to_string(), Duration::ZERO);
    }

    pub fn end_scope(&mut self) {}

    pub fn reset(&mut self) {
        self.entries.clear();
    }

    pub fn format_report(&self) -> String {
        "Profiler report".to_string()
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}