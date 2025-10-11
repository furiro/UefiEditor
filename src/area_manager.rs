use crate::area_info::{self, AreaInfo, BinArea, VariableArea};


pub struct AreaManager{
    pub bin_area        : BinArea,
    pub variable_area   : VariableArea,
}


impl AreaManager {
    pub fn new() -> AreaManager {

        let var_pos: [usize; 2] = [0,0];
        let variable_area = area_info::VariableArea::new(AreaInfo{ pos : var_pos , hight: 5, widht: 48, cursor_offset: [0, 0] });

        let bin_area_hight = 5;    // get from gop.mode
        let bin_pos: [usize; 2] = [0, variable_area.area_info.hight];
        let bin_area: BinArea = area_info::BinArea::new(AreaInfo{ pos : bin_pos , hight: bin_area_hight, widht: 48, cursor_offset: [9, 1] });

        Self {
            bin_area,
            variable_area,
        }
    }
}
