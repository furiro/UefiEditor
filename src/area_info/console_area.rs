
//
// Console
//
use core::fmt::Write;

use uefi::{boot, proto::console::text::{Color, Key, Output}, CStr16};

use crate::{area_info::area_info::{AreaInfo, DrawArea}, editor_info::EditorInfo, uefi_editor::Cmd};

pub struct ConsoleArea {
    pub area_info       : AreaInfo,
    pub have_update     : bool,
}

impl ConsoleArea {
    pub fn new(area_info:AreaInfo) -> ConsoleArea{

        Self {
            area_info,
            have_update : true,
        }
    }
}

impl DrawArea for ConsoleArea {
    fn input_handle(&self, key:Key) -> (Cmd, i32) {
        let operation:(Cmd, i32);
        match key {
            uefi::proto::console::text::Key::Printable(p) => {
                let temp:u16 = p.into();
                operation = (Cmd::WriteInputBuffer, temp as i32);
            }
            _ => operation = (Cmd::NoOp, 0),
        }

        return operation;
    }
    fn draw(&mut self, output_protocol:  &mut boot::ScopedProtocol<Output>, editor_info:& EditorInfo) {
        let _ = output_protocol.set_color(Color::White, Color::Black);
        let _ = output_protocol.set_cursor_position(self.area_info.pos[0], self.area_info.pos[1]);

        let cstr = CStr16::from_char16_with_nul(&editor_info.input_buffer).ok();

        let _ = writeln! (output_protocol, "Console Area");

        match cstr {
            Some(s) => {let _ = writeln!(output_protocol, "{}", s);},
            _ => (),
        }
        self.have_update = false;
    }

}

