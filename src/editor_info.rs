
use core::fmt::Write;

use uefi::{boot, proto::console::text::{Color, Key, Output, ScanCode}, CStr16, Status};

use crate::variable::VariableInfo;
use crate::area_info::AreaInfo;

pub struct EditorInfo<'a>{
    pub offset              : usize,
    pub output_protocol     : boot::ScopedProtocol<Output>,
    pub var_info            : VariableInfo<'a>,
    pub is_low_bit          : usize,
    pub bin_area            : AreaInfo,
    pub address_offset      : usize,
    pub need_rewrite        : bool,
}

pub fn char16_to_hex(byte: u16) -> Option<u8> {
    let c = char::from_u32(byte as u32)?;

    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'A'..='F' => Some(c as u8 - b'A' + 10),
        'a'..='f' => Some(c as u8 - b'a' + 10),
        _ => None,
    }
}

impl <'a>  EditorInfo<'a> {

    pub fn new(output_protocol : boot::ScopedProtocol<Output>, var_info : VariableInfo<'a>) -> EditorInfo<'a> {
        let def_pos: [usize; 2] = [0,5];

        let bin_area_hight = 5;    // get from gop.mode

        Self {
            offset              : 0,
            output_protocol     : output_protocol,
            var_info            : var_info,
            is_low_bit          : 0,
            address_offset      : 0,
            bin_area            : AreaInfo{ pos: [def_pos[0] + 9    , def_pos[1] + 1]  , hight: bin_area_hight      , widht: 48     , cursor_offset: [0,0] },
            need_rewrite         : true,
        }
    }

    pub fn write_all_area(&mut self) {
        // self.write_address_area();
        // self.write_ascii_area();
        // self.write_bin_area();
        self.write_bin_are();
        self.need_rewrite = false;
    }

    pub fn write_bin_are (&mut self) {
        let _ = self.output_protocol.enable_cursor(false);
        let _ = self.output_protocol.set_color(Color::White, Color::Black);
        let def_pos: [usize; 2] = [0,5];
        let bin_area_hight = 5;    // get from gop.mode

        let address_low_area_pos = [def_pos[0] + 9, def_pos[1]];
        let bin_area_pos = [def_pos[0], def_pos[1] + 1];

        // write upside of bin area
        let _ = self.output_protocol.set_cursor_position(address_low_area_pos[0], address_low_area_pos[1]);
        for i in 0..=0xf {
            let _ = write!(&mut self.output_protocol, "{:02X} ", i);
        }

        for hight in 0..bin_area_hight {
            // write bin & ascii
            for i in 0..16 {
                let write_at =  (self.address_offset + hight)*16 + i;
                if write_at < self.var_info.size {
                    // bin
                    let _ = self.output_protocol.set_cursor_position(bin_area_pos[0] + 9 + i*3, bin_area_pos[1] + hight);
                    let _ = write!(self.output_protocol, "{:02X} ", self.var_info.data[write_at]);
                    // ascii
                    let _ = self.output_protocol.set_cursor_position(bin_area_pos[0] + 9 + 48 + i*3, bin_area_pos[1] + hight);
                    match CStr16::from_u16_with_nul(&[self.var_info.data[write_at] as u16,0]) {
                        Ok(c) => {let _ = write!(self.output_protocol, "{} ", c);},
                        Err(_err) => {let _ = write!(self.output_protocol, ". ");},
                    }
                } else {
                    // bin
                    let _ = self.output_protocol.set_cursor_position(bin_area_pos[0] + 9 + i*3, bin_area_pos[1] + hight);
                    let _ = write!(self.output_protocol, "   ");
                    // ascii
                    let _ = self.output_protocol.set_cursor_position(bin_area_pos[0] + 9 + 48 + i*3, bin_area_pos[1] + hight);
                    let _ = write!(self.output_protocol, "  ");
                }
            }

            if (self.address_offset + hight) * 16 >= self.var_info.size {
                break;
            }
            // write left side of bin area
            let _ = self.output_protocol.set_cursor_position(bin_area_pos[0], bin_area_pos[1] + hight);
            let _ = write!(&mut self.output_protocol, "{:08X} ", (hight + self.address_offset)*0x10);
        }

        let _ = self.output_protocol.enable_cursor(true);
    }


    fn cmd_move_to(&mut self, move_to:i32) {
        if move_to == 0 {
            return;
        }
        let movement = move_to.abs().try_into().unwrap();
        if move_to < 0 {

            if self.offset < movement {
                self.offset = 0;
            } else {
                self.offset -= movement;
                if self.offset < self.address_offset * 16 {
                    self.address_offset -= 1;
                    self.need_rewrite = true;
                }
            }
        } else {
            if self.offset + movement > self.var_info.size - 1 {
                self.offset = self.var_info.size -1;
            } else {
                self.offset += movement;
                if self.offset + movement > (self.address_offset + self.bin_area.hight) * 16 -1 {
                    self.address_offset += 1;
                    self.need_rewrite = true;
                }
            }
        }

        self.bin_area.cursor_offset = self.cursor_offset_from_offset();
    }

    fn cmd_write_at(&mut self, value:u8) {
        if self.is_low_bit == 1 {
            self.var_info.data[self.offset] = (self.var_info.data[self.offset] & (0xf0 as u8)) | value;
            self.is_low_bit = 0;
            self.offset += 1;
        } else {
            self.var_info.data[self.offset] = (self.var_info.data[self.offset] & (0x0f as u8)) | value<<4;
            self.is_low_bit = 1;
        }
        self.bin_area.cursor_offset = self.cursor_offset_from_offset();
        self.need_rewrite = true;
    }

    pub fn input_handle(&mut self, key:Key) -> Status{
        match key {
            uefi::proto::console::text::Key::Printable(p) if u16::from(p) <= 0x1a => {
                // ctrl pressed
                match u16::from(p) {
                    // ctrl + s
                    0x13 => {
                        let _ = self.var_info.save();
                        return Status::ABORTED
                    },
                    _ => (),
                }
            }
            uefi::proto::console::text::Key::Printable(p) => {
                match char16_to_hex(u16::from(p)) {
                    Some(x) => {
                        self.cmd_write_at(x);
                    },
                    None => (),
                }
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::ESCAPE => {
                return Status::ABORTED;
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::UP => {
                self.cmd_move_to(-16);
                self.bin_area.cursor_offset = self.cursor_offset_from_offset();
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::LEFT => {

                self.cmd_move_to(-1);
                self.bin_area.cursor_offset = self.cursor_offset_from_offset();
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::RIGHT => {
                self.cmd_move_to(1);
                self.bin_area.cursor_offset = self.cursor_offset_from_offset();
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::DOWN => {
                self.cmd_move_to(16);
                self.bin_area.cursor_offset = self.cursor_offset_from_offset();
            }
            uefi::proto::console::text::Key::Special(_s) => {
            }
        }
        return Status::SUCCESS
    }

    fn cursor_offset_from_offset(&mut self) -> [usize;2] {
        [self.offset%16*3 + self.is_low_bit, self.offset/16 - self.address_offset]
    }
}
