pub struct SweepingIterator {
    start: i32,
    end: i32,
    current: i32,
    direction: i8
}

impl SweepingIterator {
    pub fn new(end: i32) -> SweepingIterator {
        SweepingIterator {start: 0, end: end, current: 0, direction: 1}
    }
}

impl Iterator for SweepingIterator {
    // we will be counting with usize
    type Item = i32;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        
        self.current += self.direction as i32;
        if self.current >= self.end || self.current <= self.start {self.direction *= -1}

        Some(self.current)
    }
}