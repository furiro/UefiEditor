


use crate::variable::VariableInfo;

pub struct EditorInfo<'a>{
    pub offset              : usize,
    pub var_info            : VariableInfo<'a>,
    pub is_low_bit          : usize,
    pub address_offset      : usize,
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

        Self {
            offset              : 0,
            var_info            : var_info,
            is_low_bit          : 0,
            address_offset      : 0,
        }
    }

}
