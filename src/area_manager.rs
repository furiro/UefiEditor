use crate::area_info::{self, AreaInfo, BinArea};


pub struct AreaManager{
    pub bin_area : BinArea,
}


impl AreaManager {
    pub fn new() -> AreaManager {

        let bin_area_hight = 5;    // get from gop.mode
        let pos: [usize; 2] = [0,5];
        let bin_area: BinArea = area_info::BinArea::new(AreaInfo{ pos , hight: bin_area_hight, widht: 48, cursor_offset: [9,1] });

        Self {
            bin_area,
        }
    }
}
