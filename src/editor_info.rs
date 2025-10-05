
use core::fmt::Write;

use uefi::{boot, proto::console::text::{Color, Key, Output, ScanCode}, Status};

use crate::variable::VariableInfo;

pub struct EditorInfo<'a>{
    pub offset          : usize,
    pub output_protocol : boot::ScopedProtocol<Output>,
    pub var_info        : VariableInfo<'a>,
    pub is_low_bit      : usize,
    pub def_pos         : [usize;2]
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

    pub fn write_all_area(&mut self) {
        self.write_address_area();
        self.write_bin_area();
    }

    pub fn write_address_area(&mut self) {

        let address_low_pos:[usize;2]   = [self.def_pos[0] + 9   , self.def_pos[1]];
        let address_high_pos:[usize;2]  = [self.def_pos[0]       , self.def_pos[1] + 1];

        // write upside of bin area
        let _ = self.output_protocol.set_cursor_position(address_low_pos[0], address_low_pos[1]);
        let _ = self.output_protocol.set_color(Color::White, Color::Black);
        for i in 0..=0xf {
            let _ = write!(&mut self.output_protocol, "{:02X} ", i);
        }

        // write left side of bin area
        let _ = self.output_protocol.set_color(Color::White, Color::Black);
        for i in 0..=self.var_info.size/16 {
            let _ = self.output_protocol.set_cursor_position(address_high_pos[0], address_high_pos[1] + i);
            let _ = write!(&mut self.output_protocol, "{:08X} ", i*0x10);
        }
    }

    pub fn write_bin_area(&mut self) {
        let bin_pos:[usize;2] = [self.def_pos[0] + 9, self.def_pos[1] + 1];
        let _ = self.output_protocol.enable_cursor(false);
        let _ = self.output_protocol.set_cursor_position(bin_pos[0], bin_pos[1]);
        let _ = self.output_protocol.set_color(Color::Yellow, Color::Black);

        for (i, b) in self.var_info.data.iter().enumerate() {
            let _ = write!(self.output_protocol, "{:02X} ", b);
            if (i + 1) % 16 == 0 {
                let _ = self.output_protocol.set_cursor_position(bin_pos[0], bin_pos[1] + ((i + 1) / 16));
            }
        }

        let _ = self.output_protocol.enable_cursor(true);
        let _ = self.output_protocol.set_cursor_position(bin_pos[0] + (self.offset%16 )*3 + self.is_low_bit, bin_pos[1] + self.offset/16);

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
                let _ = self.output_protocol.set_cursor_position(0, 0);

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
                }
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::LEFT => {
                if self.offset < 1 {
                    self.offset = 0;
                } else {
                    self.offset -= 1;
                }
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::RIGHT => {
                if self.offset + 1 > self.var_info.size - 1 {
                    self.offset = self.var_info.size -1;
                } else {
                    self.offset += 1;
                }
            }
            uefi::proto::console::text::Key::Special(s) if s == ScanCode::DOWN => {
                if self.offset + 16 > self.var_info.size - 1 {
                    self.offset = self.var_info.size -1;
                } else {
                    self.offset += 16;
                }
            }
            uefi::proto::console::text::Key::Special(_s) => {
            }
        }
        return Status::SUCCESS
    }
}
