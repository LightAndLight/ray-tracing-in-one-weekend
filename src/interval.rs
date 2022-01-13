pub struct Interval {
    pub start: f64,
    pub end: f64,
}

impl Interval {
    pub fn intersect_mut(&mut self, other: &Interval) {
        self.start = self.start.max(other.start);
        self.end = self.end.min(other.end);
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}
