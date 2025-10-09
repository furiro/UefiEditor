
use uefi::proto::console::text::{self, Key};
use crate::editor_info::EditorInfo;

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

enum cmd { write_all, write_at, move_to, goto, }

pub trait DraArea {
    fn InputKey(&self, key:Key) -> (cmd, usize);
    fn Draw(&self, ei:EditorInfo);
}

struct BinArea{
    
}

impl DraArea for BinArea { 
    fn InputKey(&self, key:Key) -> (cmd, usize) 
    {
        return (cmd::write_all, 0);
    }
    fn Draw(&self, ei:EditorInfo){
        
    } 
}

