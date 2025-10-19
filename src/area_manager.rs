use uefi::proto::console::text::ScanCode;

use crate::area_info::area_info::{AreaInfo, DrawArea};
use crate::area_info::bin_area::BinArea;
use crate::area_info::console_area::ConsoleArea;
use crate::area_info::variable_info_area::VariableArea;
use crate::constants::*;
use crate::input_ex::{EfiKeyData, KeyShiftState};
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

        let var_pos: [usize; 2]         = [0,0];
        let variable_area: VariableArea = VariableArea::new(AreaInfo{ pos : var_pos , hight: 5, widht: 48, cursor_offset: [0, 0] });

        let bin_area_hight: usize   = 5;    // get from gop.mode
        let bin_pos: [usize; 2]     = [var_pos[0], var_pos[1] + variable_area.area_info.hight];
        let bin_area: BinArea       = BinArea::new(AreaInfo{ pos : bin_pos , hight: bin_area_hight, widht: 48, cursor_offset: [BIN_AREA_CURSOR_DEFAULT_X, BIN_AREA_CURSOR_DEFAULT_Y] });

        let console_hight: usize        = 4;
        let console_pos  : [usize;2]    = [bin_pos[0], bin_pos[1] + bin_area.area_info.hight];
        let console_area : ConsoleArea  = ConsoleArea::new(AreaInfo { pos: console_pos, hight: console_hight, widht: 48, cursor_offset: [0, 1] });

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

    pub fn input_handle (&mut self, key:EfiKeyData) -> (Cmd, i32) {
        let operation:(Cmd, i32);
        match key.key.scan_code {
            // ins
            ScanCode::INSERT => operation = (Cmd::NextWindow, 0),
            // ctrl + s
            _ if key.key_state.key_shift_state.intersects(KeyShiftState::LEFT_SHIFT | KeyShiftState::RIGHT_SHIFT) => operation = (Cmd::Save, 0),
            // others
            _ => match self.active_window {
                ActiveWindow::ActiveBinArea     => operation = self.bin_area.input_handle(key),
                ActiveWindow::ActiveConsoleArea => operation = self.console_area.input_handle(key),
                //_ => operation = (Cmd::NoOp, 0),
            },
        }
        return operation;
    }

}
