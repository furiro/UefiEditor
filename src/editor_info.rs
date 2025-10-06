
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
    pub address_low_area    : AreaInfo,
    pub address_high_area   : AreaInfo,
    pub address_offset      : usize,
    pub ascii_area          : AreaInfo,
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
        let def_pos = [0,5];

        let bin_area_hight = 5;    // get from gop.mode

        Self {
            offset              : 0,
            output_protocol     : output_protocol,
            var_info            : var_info,
            is_low_bit          : 0,
            address_offset      : 0,
            bin_area            : AreaInfo{ pos: [def_pos[0] + 9    , def_pos[1] + 1]  , hight: bin_area_hight      , widht: 48     , cursor_offset: [0,0] },
            address_low_area    : AreaInfo{ pos: [def_pos[0] + 9    , def_pos[1]]      , hight: 1                   , widht: 48 + 9 , cursor_offset: [0,0] },
            address_high_area   : AreaInfo{ pos: [def_pos[0]        , def_pos[1] + 1]  , hight: bin_area_hight      , widht: 48 + 9 , cursor_offset: [0,0] },
            ascii_area          : AreaInfo{ pos: [def_pos[0] + 57   , def_pos[1] + 1]  , hight: bin_area_hight      , widht: 16     , cursor_offset: [0,0] },
        }
    }

    pub fn write_all_area(&mut self) {
        self.write_address_area();
        self.write_ascii_area();
        self.write_bin_area();
    }

    pub fn write_address_area(&mut self) {
        let _ = self.output_protocol.set_color(Color::White, Color::Black);

        // write upside of bin area
        let _ = self.output_protocol.set_cursor_position(self.address_low_area.pos[0], self.address_low_area.pos[1]);
        for i in 0..=0xf {
            let _ = write!(&mut self.output_protocol, "{:02X} ", i);
        }

        // write left side of bin area
        for hight in 0..self.address_high_area.hight {
            let _ = self.output_protocol.set_cursor_position(self.address_high_area.pos[0], self.address_high_area.pos[1] + hight);
            if (self.address_offset + hight) * 16 >= self.var_info.size {
                break;
            }
            let _ = write!(&mut self.output_protocol, "{:08X} ", (hight + self.address_offset)*0x10);
        }
    }

    pub fn write_bin_area(&mut self) {
        let _ = self.output_protocol.enable_cursor(false);
        let _ = self.output_protocol.set_color(Color::Yellow, Color::Black);

        for hight in 0..self.bin_area.hight {
            let _ = self.output_protocol.set_cursor_position(self.bin_area.pos[0], self.bin_area.pos[1] + hight);
            if (self.address_offset + hight) * 16 >= self.var_info.size {
                break;
            }
            for i in 0..16 {
                let write_at =  (self.address_offset + hight)*16 + i;
                if write_at < self.var_info.size {
                    let _ = write!(self.output_protocol, "{:02X} ", self.var_info.data[write_at]);
                } else {
                    let _ = write!(self.output_protocol, "   ");
                }
            }
        }
    }

    pub fn write_ascii_area(&mut self) {

        for hight in 0..self.ascii_area.hight {
            let _ = self.output_protocol.set_cursor_position(self.ascii_area.pos[0], self.ascii_area.pos[1] + hight);
            if (self.address_offset + hight) * 16 >= self.var_info.size {
                break;
            }
            for i in 0..16 {
                let write_at =  (self.address_offset + hight)*16 + i;
                if write_at < self.var_info.size {
                    match CStr16::from_u16_with_nul(&[self.var_info.data[write_at] as u16,0]) {
                        Ok(c) => {let _ = write!(self.output_protocol, "{} ", c);},
                        Err(_err) => {let _ = write!(self.output_protocol, ". ");},
                    }
                } else {
                    let _ = write!(self.output_protocol, "  ");
                }
            }
        }
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
                        if self.is_low_bit == 1 {
                            self.var_info.data[self.offset] = (self.var_info.data[self.offset] & (0xf0 as u8)) | x;
                            self.is_low_bit = 0;
                            self.offset += 1;
                        } else {
                            self.var_info.data[self.offset] = (self.var_info.data[self.offset] & (0x0f as u8)) | x<<4;
                            self.is_low_bit = 1;
                        }
                        self.bin_area.cursor_offset = self.cursor_offset_from_offset();
                    },
                    None => (),
                }
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::ESCAPE => {
                return Status::ABORTED;
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::UP => {
                if self.offset < 16 {
                    self.offset = 0;
                } else {
                    self.offset -= 16;
                    if self.offset < self.address_offset * 16 {
                        self.address_offset -= 1;
                    }
                }
                self.bin_area.cursor_offset = self.cursor_offset_from_offset();
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::LEFT => {

                if self.offset < 1 {
                    self.offset = 0;
                } else {
                    self.offset -= 1;
                    if self.offset < self.address_offset * 16 {
                        self.address_offset -= 1;
                    }
                }
                self.bin_area.cursor_offset = self.cursor_offset_from_offset();
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::RIGHT => {
                if self.offset + 1 > self.var_info.size - 1 {
                    self.offset = self.var_info.size -1;
                } else {
                    self.offset += 1;
                    if self.offset + 1 > (self.address_offset + self.bin_area.hight) * 16 {
                        self.address_offset += 1;
                    }
                }
                self.bin_area.cursor_offset = self.cursor_offset_from_offset();
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::DOWN => {
                if self.offset + 16 > self.var_info.size - 1 {
                    self.offset = self.var_info.size -1;
                } else {
                    self.offset += 16;
                    if self.offset > (self.address_offset + self.bin_area.hight) * 16 - 1 {
                        self.address_offset += 1;
                    }
                }
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
