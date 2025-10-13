use uefi::proto::console::text::Key;

use crate::area_info::area_info::{self, AreaInfo, DrawArea};
use crate::area_info::bin_area::BinArea;
use crate::area_info::console_area::ConsoleArea;
use crate::area_info::variable_info_area::VariableArea;
use crate::constants::*;
use crate::uefi_editor::{ Cmd};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveWindow { ActiveBinArea, ActiveConsoleArea}

impl ActiveWindow {
    const ALL: [ActiveWindow; 2] = [ActiveWindow::ActiveBinArea, ActiveWindow::ActiveConsoleArea,];
    pub fn next(self) -> ActiveWindow {
        let i = Self::ALL.iter().position(|&x| x == self).unwrap();
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
}

pub struct AreaManager{
    pub bin_area        : BinArea,
    pub variable_area   : VariableArea,
    pub console_area    : ConsoleArea,
    pub active_window   : ActiveWindow,
}


impl AreaManager {
    pub fn new() -> AreaManager {

        let var_pos: [usize; 2] = [0,0];
        let variable_area = VariableArea::new(AreaInfo{ pos : var_pos , hight: 5, widht: 48, cursor_offset: [0, 0] });

        let bin_area_hight = 5;    // get from gop.mode
        let bin_pos: [usize; 2] = [var_pos[0], var_pos[1] + variable_area.area_info.hight];
        let bin_area: BinArea = BinArea::new(AreaInfo{ pos : bin_pos , hight: bin_area_hight, widht: 48, cursor_offset: [BIN_AREA_CURSOR_DEFAULT_X, BIN_AREA_CURSOR_DEFAULT_Y] });

        let console_hight = 4;
        let console_pos :[usize;2] = [bin_pos[0], bin_pos[1] + bin_area.area_info.hight];
        let console_area : ConsoleArea = ConsoleArea::new(AreaInfo { pos: console_pos, hight: console_hight, widht: 48, cursor_offset: [0, 1] });

        Self {
            bin_area,
            variable_area,
            console_area,
            active_window   : ActiveWindow::ActiveBinArea,
        }
    }

    pub fn cursor_pos (&self) -> [usize;2] {
        match self.active_window {
            ActiveWindow::ActiveBinArea     => self.bin_area.area_info.cursor_pos(),
            ActiveWindow::ActiveConsoleArea => self.console_area.area_info.cursor_pos(),
        }
    }

    pub fn input_handle (&mut self, key:Key) -> (Cmd, i32) {
        let operation:(Cmd, i32);
        match key {
            uefi::proto::console::text::Key::Printable(p) if u16::from(p) <= 0x1a => {
                // ctrl pressed
                match u16::from(p) {
                    // ctrl + s
                    0x13 => operation = (Cmd::Save, 0),
                    // tab
                    0x09 => operation = (Cmd::NextWindow, 0),
                    _ => operation = (Cmd::NoOp, 0),
                }
                // remove when save cmd
            }
            
            _ => match self.active_window {
                ActiveWindow::ActiveBinArea     => operation = self.bin_area.input_handle(key),
                ActiveWindow::ActiveConsoleArea => operation = self.console_area.input_handle(key),
                //_ => operation = (Cmd::NoOp, 0),
            }
        }

        return operation;
    }

}
