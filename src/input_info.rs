use alloc::vec::Vec;
use uefi::Char16;

use crate::common::address_offset::AddressOffset;

#[repr(i32)]
pub enum InputMode {
    Default, Goto,
}

pub struct InputBufferInfo {
    pub offset        : AddressOffset,
    pub buffer        : Vec<Char16>,
    pub mode          : InputMode,
}

impl InputBufferInfo {
    pub fn new() -> Self{

        let mut input_buffer: Vec<Char16> = Vec::new();
        input_buffer.push(unsafe { Char16::from_u16_unchecked(0) });

        Self {
            offset        : AddressOffset::new(0, input_buffer.len()-1, 0),   // 0,0,0
            buffer        : input_buffer,
            mode          : InputMode::Default,
        }
    }
}

