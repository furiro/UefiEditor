
use core::fmt::Write;

use alloc::string::ToString;
use uefi::{boot::{self}, proto::console::text::{Color, Key, Output, ScanCode}, CStr16};
use crate::{editor_info::{char16_to_hex, EditorInfo}, uefi_editor::Cmd};

pub struct AreaInfo{
    pub(crate) pos      : [usize;2],
    pub hight           : usize,
    pub widht           : usize,
    pub cursor_offset   : [usize;2],    //offset_from pos
}

impl AreaInfo {
    pub fn cursor_pos(&mut self) -> [usize;2]{
        return [self.pos[0] + self.cursor_offset[0], self.pos[1] + self.cursor_offset[1]]
    }
}


pub trait DraArea {
    fn input_handle(&self, key:Key) -> (Cmd, i32);
    fn draw(&mut self, output_protocol:  &mut boot::ScopedProtocol<Output>, editor_info:&mut EditorInfo);
}

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
}

impl DraArea for BinArea { 
    fn input_handle(&self, key:Key) -> (Cmd, i32) 
    {
        let operation:(Cmd, i32);
        match key {
            uefi::proto::console::text::Key::Printable(p) => {
                match char16_to_hex(u16::from(p)) {
                    Some(x) => {
                        operation = (Cmd::WriteAt, x as i32);
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

    fn draw(&mut self, mut output_protocol: &mut boot::ScopedProtocol<Output>, editor_info:&mut EditorInfo){

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
                let write_at =  (editor_info.address_offset + hight)*16 + i;
                if write_at < editor_info.var_info.size {
                    // bin
                    let _ = output_protocol.set_color(Color::Yellow, Color::Black);
                    let _ = output_protocol.set_cursor_position(bin_area_pos[0] + 9 + i*3, bin_area_pos[1] + hight);
                    let _ = write!(output_protocol, "{:02X} ", editor_info.var_info.data[write_at]);
                    // ascii
                    let _ = output_protocol.set_color(Color::White, Color::Black);
                    let _ = output_protocol.set_cursor_position(bin_area_pos[0] + 9 + 48 + 1 + i*2, bin_area_pos[1] + hight);
                    match CStr16::from_u16_with_nul(&[editor_info.var_info.data[write_at] as u16,0]) {
                        Ok(c) => {let _ = write!(output_protocol, "{} ", c);},
                        Err(_err) => {let _ = write!(output_protocol, ". ");},
                    }
                } else {
                    // bin
                    let _ = output_protocol.set_color(Color::Yellow, Color::Black);
                    let _ = output_protocol.set_cursor_position(bin_area_pos[0] + 9 + i*3, bin_area_pos[1] + hight);
                    let _ = write!(output_protocol, "   ");
                    // ascii
                    let _ = output_protocol.set_color(Color::White, Color::Black);
                    let _ = output_protocol.set_cursor_position(bin_area_pos[0] + 9 + 48 + 1 + i*2, bin_area_pos[1] + hight);
                    let _ = write!(output_protocol, "  ");
                }
            }

           if (editor_info.address_offset + hight) * 16 >= editor_info.var_info.size {
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

impl DraArea for VariableArea {
    fn input_handle(&self, _:Key) -> (Cmd, i32) {
        return (Cmd::NoOp, 0);
    }
    fn draw(&mut self, output_protocol:  &mut boot::ScopedProtocol<Output>, editor_info:&mut EditorInfo) {
        let _ = output_protocol.set_color(Color::White, Color::Black);
        let _ = output_protocol.set_cursor_position(self.area_info.pos[0], self.area_info.pos[1]);
        let _ = writeln!(output_protocol, "Name : {}", editor_info.var_info.name);
        let _ = writeln!(output_protocol, "Guid : {}", editor_info.var_info.guid.to_string());
        let _ = writeln!(output_protocol, "Size : {}", editor_info.var_info.size);

        self.have_update = false;
    }
}
