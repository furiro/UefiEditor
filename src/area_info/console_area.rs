
//
// Console
//
use core::fmt::Write;

use uefi::{boot, proto::console::text::{Color, Key, Output, ScanCode}, CStr16};

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

    pub fn update_cursor_offset(&mut self, editor_info:&EditorInfo) {
        self.area_info.cursor_offset = [editor_info.input_offset.current, 1];
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
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::LEFT => {
                operation = (Cmd::MoveTo, -1);
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::RIGHT => {
                operation = (Cmd::MoveTo, 1);
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

