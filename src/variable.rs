
use alloc::vec::Vec;
use uefi::CStr16;
use uefi::prelude::*;
use uefi::runtime::{VariableAttributes, VariableVendor};


pub struct VariableInfo<'a> {
    pub name: &'a CStr16,
    pub guid: VariableVendor,
    pub attr: VariableAttributes,
    pub size: usize,
    pub data: Vec<u8>,
}

impl<'a> VariableInfo<'a> {

    pub fn init(&mut self) -> Status {

        loop {
            match uefi::runtime::get_variable(self.name, &self.guid, &mut self.data) {
                Ok((slice, attrs)) => {
                    self.attr = attrs;
                    self.size = slice.len();
                    return Status::SUCCESS
                }
                Err(err) if err.status() == Status::BUFFER_TOO_SMALL=> {
                    self.size = err.data().unwrap();
                    self.data.resize(self.size, 0);
                }
                Err(err) => {
                    return err.status();
                }
            }
        }
    }

    pub fn save(&mut self) -> Status{
        return uefi::runtime::set_variable(self.name, &self.guid, self.attr,  &self.data).status();
    }
}

