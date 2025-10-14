
//
// BinArea
//

use core::fmt::Write;

use uefi::{boot, proto::console::text::{Color, Key, Output, ScanCode}, CStr16};

use crate::{area_info::area_info::{AreaInfo, DrawArea}, constants::*, editor_info::{char16_to_hex, EditorInfo}, uefi_editor::Cmd};

pub struct BinArea{
    pub area_info       : AreaInfo,
    pub have_update     : bool
}

impl BinArea {
    pub fn new(area_info:AreaInfo) -> BinArea{

        Self {
            area_info,
            have_update : true,
        }
    }

    pub fn update_cursor_offset(&mut self, editor_info:& EditorInfo) {
        self.area_info.cursor_offset = [editor_info.var_offset.current%16*3 + editor_info.is_low_bit + BIN_AREA_CURSOR_DEFAULT_X,
                                        editor_info.var_offset.current/16 - editor_info.start_address + BIN_AREA_CURSOR_DEFAULT_Y]
    }
}

impl DrawArea for BinArea { 
    fn input_handle(&self, key:Key) -> (Cmd, i32) 
    {
        let operation:(Cmd, i32);
        match key {
            uefi::proto::console::text::Key::Printable(p) => {
                match char16_to_hex(u16::from(p)) {
                    Some(x) => {
                        operation = (Cmd::WriteVariable, x as i32);
                    },
                    None => operation = (Cmd::NoOp, 0),
                }
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::ESCAPE => {
                operation = (Cmd::Quit, 0);
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::UP => {
                operation = (Cmd::MoveTo, -16);
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::LEFT => {
                operation = (Cmd::MoveTo, -1);
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::RIGHT => {
                operation = (Cmd::MoveTo, 1);
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::DOWN => {
                operation = (Cmd::MoveTo, 16);
            }
            uefi::proto::console::text::Key::Special(_s) => {
                operation = (Cmd::NoOp, 0);
            }
        }
        return operation;
    }

    fn draw(&mut self, mut output_protocol: &mut boot::ScopedProtocol<Output>, editor_info:& EditorInfo){

        let _ = output_protocol.enable_cursor(false);

        let address_low_area_pos = [self.area_info.pos[0] + 9, self.area_info.pos[1]];
        let bin_area_pos = [self.area_info.pos[0], self.area_info.pos[1] + 1];

        // write upside of bin area
        let _ = output_protocol.set_color(Color::White, Color::Black);
        let _ = output_protocol.set_cursor_position(address_low_area_pos[0], address_low_area_pos[1]);
        for i in 0..=0xf {
            let _ = write!(&mut output_protocol, "{:02X} ", i);
        }

        for hight in 0..self.area_info.hight {
            // write bin & ascii
            for i in 0..16 {
                let write_at =  (editor_info.start_address + hight)*16 + i;
                if write_at < editor_info.var_info.size {
                    // bin
                    let _ = output_protocol.set_color(Color::Yellow, Color::Black);
                    let _ = output_protocol.set_cursor_position(bin_area_pos[0] + BIN_AREA_CURSOR_DEFAULT_X + i*3, bin_area_pos[1] + hight);
                    let _ = write!(output_protocol, "{:02X} ", editor_info.var_info.data[write_at]);
                    // ascii
                    let _ = output_protocol.set_color(Color::White, Color::Black);
                    let _ = output_protocol.set_cursor_position(bin_area_pos[0] + BIN_AREA_CURSOR_DEFAULT_X + 48 + 1 + i*2, bin_area_pos[1] + hight);
                    match CStr16::from_u16_with_nul(&[editor_info.var_info.data[write_at] as u16,0]) {
                        Ok(c) => {let _ = write!(output_protocol, "{} ", c);},
                        Err(_err) => {let _ = write!(output_protocol, ". ");},
                    }
                } else {
                    // bin
                    let _ = output_protocol.set_color(Color::Yellow, Color::Black);
                    let _ = output_protocol.set_cursor_position(bin_area_pos[0] + BIN_AREA_CURSOR_DEFAULT_X + i*3, bin_area_pos[1] + hight);
                    let _ = write!(output_protocol, "   ");
                    // ascii
                    let _ = output_protocol.set_color(Color::White, Color::Black);
                    let _ = output_protocol.set_cursor_position(bin_area_pos[0] + BIN_AREA_CURSOR_DEFAULT_X + 48 + 1 + i*2, bin_area_pos[1] + hight);
                    let _ = write!(output_protocol, "  ");
                }
            }

           if (editor_info.start_address + hight) * 16 >= editor_info.var_info.size {
                break;
            }
            // write left side of bin area
            let _ = output_protocol.set_color(Color::White, Color::Black);
            let _ = output_protocol.set_cursor_position(bin_area_pos[0], bin_area_pos[1] + hight);
            let _ = write!(&mut output_protocol, "{:08X} ", (hight + 0)*0x10);
        }

        self.have_update = false;
    } 
}

