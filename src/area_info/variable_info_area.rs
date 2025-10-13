use alloc::string::ToString;
use uefi::{boot, proto::console::text::{Color, Key, Output}};
use core::fmt::Write;

use crate::{area_info::area_info::{AreaInfo, DrawArea}, editor_info::EditorInfo, uefi_editor::Cmd};


//
// VariableInfo
//
pub struct VariableArea {
    pub area_info       : AreaInfo,
    pub have_update     : bool
}

impl VariableArea {
    pub fn new(area_info:AreaInfo) -> VariableArea{

        Self {
            area_info,
            have_update : true,
        }
    }
}

impl DrawArea for VariableArea {
    fn input_handle(&self, _:Key) -> (Cmd, i32) {
        return (Cmd::NoOp, 0);
    }
    fn draw(&mut self, output_protocol:  &mut boot::ScopedProtocol<Output>, editor_info:& EditorInfo) {
        let _ = output_protocol.set_color(Color::White, Color::Black);
        let _ = output_protocol.set_cursor_position(self.area_info.pos[0], self.area_info.pos[1]);
        let _ = writeln!(output_protocol, "Name : {}", editor_info.var_info.name);
        let _ = writeln!(output_protocol, "Guid : {}", editor_info.var_info.guid.to_string());
        let _ = writeln!(output_protocol, "Size : {}", editor_info.var_info.size);

        self.have_update = false;
    }
}

