
pub struct Number {
    pub value: u64,
}

impl Number {
    pub fn double(&self) -> u64 {
        self.value * 2
    }

    pub fn tripple(&self) -> u64 {
        self.value * 3
    }
}
