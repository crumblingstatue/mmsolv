pub struct IncWrap {
    pub value: u8,
    min: u8,
    max: u8,
}

impl IncWrap {
    pub fn new(min: u8, max: u8) -> Self {
        Self {
            value: min,
            min,
            max,
        }
    }
    pub fn inc(&mut self) {
        self.value += 1;
        if self.value > self.max {
            self.value = self.min;
        }
    }
}
