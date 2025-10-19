



use crate::{common::address_offset::AddressOffset, input_info::InputBufferInfo, variable::VariableInfo};

pub struct EditorInfo<'a>{
    pub var_offset          : AddressOffset,
    pub var_info            : VariableInfo<'a>,
    pub input_info          : InputBufferInfo,
    pub is_low_bit          : usize,
    pub start_address       : usize,
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

    pub fn new(var_info : VariableInfo<'a>) -> EditorInfo<'a> {


        let var_offset_max:usize;
        if var_info.size < 1 {
            var_offset_max = 0;
        } else {
            var_offset_max = var_info.size - 1;
        }
        Self {
            var_offset          : AddressOffset::new(0, var_offset_max, 0),
            var_info            : var_info,
            input_info          : InputBufferInfo::new(),
            is_low_bit          : 0,
            start_address       : 0,
        }
    }

}
