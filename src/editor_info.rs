



use alloc::vec::Vec;
use uefi::Char16;

use crate::{common::address_offset::AddressOffset, variable::VariableInfo};

pub struct EditorInfo<'a>{
    pub var_offset          : AddressOffset,
    pub var_info            : VariableInfo<'a>,
    pub input_offset        : AddressOffset,
    pub input_buffer        : Vec<Char16>,
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

        let mut input_buffer: Vec<Char16> = Vec::new();
        input_buffer.push(unsafe { Char16::from_u16_unchecked(0) });

        Self {
            var_offset          : AddressOffset::new(0, var_info.size, 0),
            var_info            : var_info,
            input_offset        : AddressOffset::new(0, input_buffer.len()-1, 0),   // 0,0,0
            input_buffer        : input_buffer,
            is_low_bit          : 0,
            start_address       : 0,
        }
    }

}
