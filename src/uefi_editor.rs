
use uefi::boot::{self};
use uefi::proto::console::text::{Key, Output};
use uefi::Status;

use crate::area_info::DraArea;
use crate::editor_info::EditorInfo;
use crate::area_manager::AreaManager;
use crate::variable::VariableInfo;

pub enum Cmd { WriteAll, WriteAt, MoveTo, Goto, Quit, Save, NoOp}

pub struct UefiEditor<'a> {
    pub editor_info     : EditorInfo<'a>,
    pub area_manager    : AreaManager,
    pub output_protocol : boot::ScopedProtocol<Output>,
    pub is_quit         : bool,
}


impl <'a>UefiEditor<'a> {
    pub fn new(var_info : VariableInfo<'a>, output_protocol : boot::ScopedProtocol<Output> ) -> UefiEditor<'a> {

        let editor_info = EditorInfo::new(var_info);
        let area_manager = AreaManager::new();



        Self {
            editor_info,
            area_manager,
            output_protocol,
            is_quit : false,
        }
    }

    pub fn draw(&mut self) {
        if self.area_manager.bin_area.have_update {
            self.area_manager.bin_area.draw(&mut self.output_protocol, &mut self.editor_info);
        }
    }

    pub fn clear (&mut self){
        let _ = self.output_protocol.clear();
    }

    pub fn update_cursor(&mut self) {
        let current_cursor = self.area_manager.bin_area.area_info.cursor_pos();  // ef.active_area.cursor_pos()
        let _ = self.output_protocol.set_cursor_position(current_cursor[0], current_cursor[1]);
        let _ = self.output_protocol.enable_cursor(true);

    }

    pub fn init(&mut self) {
        let _ = self.output_protocol.clear();

        self.area_manager.bin_area.draw(&mut self.output_protocol, &mut self.editor_info);
    }

    pub fn input_handle(&mut self, key:Key) -> Status{
        let operation:(Cmd, i32);

        match key {
            uefi::proto::console::text::Key::Printable(p) if u16::from(p) <= 0x1a => {
                // ctrl pressed
                match u16::from(p) {
                    // ctrl + s
                    0x13 => {
                        operation = (Cmd::Save, 0);
                    },
                    _ => operation = (Cmd::NoOp, 0),
                }
                // remove when save cmd
            }

            _ => operation = self.area_manager.bin_area.input_handle(key),

        }

        match operation.0 {
            Cmd::MoveTo     => self.cmd_move_to(operation.1),
            Cmd::WriteAt    => self.cmd_write_at(operation.1),
            Cmd::Goto       => (),
            Cmd::WriteAll   => (),
            Cmd::Quit       => self.cmd_quit(operation.1),
            Cmd::Save       => self.cmd_save(operation.1),
            Cmd::NoOp       => (),
            _ => (),
        }

        return Status::SUCCESS
    }

    fn cmd_save(&mut self, _:i32) {
        let _ = self.editor_info.var_info.save();
    }

    fn cmd_quit (&mut self, _:i32) {
        self.is_quit = true;
    }

    fn cmd_move_to(&mut self, move_to:i32) {
        if move_to == 0 {
            return;
        }
        let movement = move_to.abs().try_into().unwrap();
        if move_to < 0 {

            if self.editor_info.offset < movement {
                self.editor_info.offset = 0;
            } else {
                self.editor_info.offset -= movement;
                if self.editor_info.offset < self.editor_info.address_offset * 16 {
                    self.editor_info.address_offset -= 1;
                    self.area_manager.bin_area.have_update = true;
                }
            }
        } else {
            if self.editor_info.offset + movement > self.editor_info.var_info.size - 1 {
                self.editor_info.offset = self.editor_info.var_info.size -1;
            } else {
                self.editor_info.offset += movement;
                if self.editor_info.offset + movement > (self.editor_info.address_offset + self.area_manager.bin_area.area_info.hight) * 16 -1 {
                    self.editor_info.address_offset += 1;
                    self.area_manager.bin_area.have_update = true;
                }
            }
        }

        self.area_manager.bin_area.area_info.cursor_offset = self.cursor_offset_from_offset();
    }

    fn cmd_write_at(&mut self, value:i32) {
        if self.editor_info.is_low_bit == 1 {
            self.editor_info.var_info.data[self.editor_info.offset] = (self.editor_info.var_info.data[self.editor_info.offset] & (0xf0 as u8)) | value as u8;
            self.editor_info.is_low_bit = 0;
            self.editor_info.offset += 1;
        } else {
            self.editor_info.var_info.data[self.editor_info.offset] = (self.editor_info.var_info.data[self.editor_info.offset] & (0x0f as u8)) | (value<<4) as u8;
            self.editor_info.is_low_bit = 1;
        }
        self.area_manager.bin_area.area_info.cursor_offset = self.cursor_offset_from_offset();
        self.area_manager.bin_area.have_update = true;
    }

    fn cursor_offset_from_offset(&mut self) -> [usize;2] {
        [self.editor_info.offset%16*3 + self.editor_info.is_low_bit + 9, self.editor_info.offset/16 - self.editor_info.address_offset + 1]
    }
}




