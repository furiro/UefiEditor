
pub struct AddressOffset{
    min         : usize,
    max         : usize,
    pub current : usize,
}

impl AddressOffset {
    pub fn new(min:usize, max:usize, offset:usize)-> AddressOffset {
        Self {
            min,
            max,
            current: offset,
        }
    }
    pub fn increase(&mut self, value:usize) {
        if self.current + value > self.max {
            self.current = self.max;
        } else {
            self.current += value;
        }
    }
    pub fn decrease(&mut self, value:usize) {
        if self.current < self.min + value {
            self.current = self.min;
        } else {
            self.current -= value;
        }
    }
}
