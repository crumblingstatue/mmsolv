/// Loops over predetermined values
pub struct ValLooper<'v> {
    source: &'v [u8],
    idx: usize,
}

impl<'v> ValLooper<'v> {
    pub fn new(source: &'v [u8]) -> Self {
        Self { source, idx: 0 }
    }
    pub fn go_next(&mut self) {
        self.idx += 1;
        if self.idx >= self.source.len() {
            self.idx = 0;
        }
    }
    pub fn value(&self) -> u8 {
        self.source[self.idx]
    }
}
