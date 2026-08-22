pub struct MemoryTracker;

impl MemoryTracker {
    pub fn new() -> Self {
        Self
    }

    pub fn allocate(&self, _size: usize, _tag: &str) {}
    pub fn deallocate(&self, _size: usize) {}
    pub fn reset(&self) {}
}