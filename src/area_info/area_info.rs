

use uefi::{boot::{self}, proto::console::text::{Key, Output}};
use crate::{editor_info::EditorInfo, uefi_editor::Cmd};

pub struct AreaInfo{
    pub(crate) pos      : [usize;2],
    pub hight           : usize,
    pub widht           : usize,
    pub cursor_offset   : [usize;2],    //offset_from pos
}

impl AreaInfo {
    pub fn cursor_pos(&self) -> [usize;2]{
        return [self.pos[0] + self.cursor_offset[0], self.pos[1] + self.cursor_offset[1]]
    }
}


pub trait DrawArea {
    fn input_handle(&self, key:Key) -> (Cmd, i32);
    fn draw(&mut self, output_protocol:  &mut boot::ScopedProtocol<Output>, editor_info:& EditorInfo);
}


