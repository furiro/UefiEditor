

pub struct AreaInfo{
    pub(crate) pos      : [usize;2],
    pub hight           : usize,
    pub widht           : usize,
    pub cursor_offset    : [usize;2],    //offset_from pos
}

impl AreaInfo {
    pub fn cursor_pos(&mut self) -> [usize;2]{
        return [self.pos[0] + self.cursor_offset[0], self.pos[1] + self.cursor_offset[1]]
    }
}

