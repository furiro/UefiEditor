
//
// Console
//
use core::fmt::Write;

use uefi::{boot, proto::console::text::{Color, Output, ScanCode}, CStr16, Char16};

use crate::{area_info::area_info::{AreaInfo, DrawArea}, editor_info::EditorInfo, input_ex::EfiKeyData, input_info::InputMode, uefi_editor::Cmd};

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
        self.area_info.cursor_offset = [editor_info.input_info.offset.current, 1];
    }
}

impl DrawArea for ConsoleArea {
    fn input_handle(&self, key:EfiKeyData) -> (Cmd, i32) {
        let operation:(Cmd, i32);

        match Char16::try_from(key.key.unicode_char) {
            // printable
            Ok(c) => operation = (Cmd::WriteInputBuffer, u16::from(c) as i32),
            Err(_) => {
                match key.key.scan_code {
                    ScanCode::LEFT => operation = (Cmd::MoveTo, -1),
                    ScanCode::RIGHT => operation = (Cmd::MoveTo, 1),
                    // others
                    _ => operation = (Cmd::NoOp, 0),
                }
            },
        }
        return operation;
    }

    fn draw(&mut self, output_protocol:  &mut boot::ScopedProtocol<Output>, editor_info:& EditorInfo) {
        let _ = output_protocol.set_color(Color::White, Color::Black);
        let _ = output_protocol.set_cursor_position(self.area_info.pos[0], self.area_info.pos[1]);
        let _ = writeln!(output_protocol, "{}", " ".repeat(self.area_info.widht));
        let _ = output_protocol.set_cursor_position(self.area_info.pos[0], self.area_info.pos[1]);
        match editor_info.input_info.mode {
            InputMode::Default  => {let _ = writeln! (output_protocol, "Console Area");},
            InputMode::Goto     => {let _ = writeln! (output_protocol, "Go to");},
        }

        let cstr = CStr16::from_char16_with_nul(&editor_info.input_info.buffer).ok();

        match cstr {
            Some(s) => {
                let _ = output_protocol.set_cursor_position(self.area_info.pos[0], self.area_info.pos[1] + 1);
                let _ = writeln!(output_protocol, "{}", " ".repeat(self.area_info.widht));
                let _ = output_protocol.set_cursor_position(self.area_info.pos[0], self.area_info.pos[1] + 1);
                let _ = writeln!(output_protocol, "{}", s);
            },
            _ => (),
        }
        self.have_update = false;
    }

}

