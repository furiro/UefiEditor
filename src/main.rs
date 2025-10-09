#![no_std]
#![no_main]

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::loaded_image::LoadedImage;
use uefi::{prelude::*, print, CStr16, Event};
use uefi::proto::console::text::{Color, Input, Output};
use uefi::{Guid, cstr16};
use uefi::boot::{self, get_handle_for_protocol};
use uefi::runtime::{VariableVendor, VariableAttributes};


extern crate alloc;

mod variable;
mod editor_info;
mod area_info;


fn make_test_variable() -> Status {

    let var_name = cstr16!("MyVar");
    let vendor_guid = Guid::from_bytes([
        0x78, 0x56, 0x34, 0x12,
        0x34, 0x12,
        0x78, 0x56,
        0x12, 0x34,
        0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    ]);
    let vendor: VariableVendor = VariableVendor(vendor_guid);
    let data: &[u8] = b"01234567890123456789012345678901234567890123456789";
    let attrs: VariableAttributes =   VariableAttributes::BOOTSERVICE_ACCESS
                                    | VariableAttributes::RUNTIME_ACCESS;

    let status = uefi::runtime::set_variable(var_name, &vendor, attrs, & data);
    status.status()
}

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();

    //
    // prepare protocols
    //
    let loaded_image_protocol: boot::ScopedProtocol<LoadedImage> = match boot::open_protocol_exclusive:: <LoadedImage>(uefi::boot::image_handle()) {
        Ok(p) => p,
        Err(err) => return err.status(),
    };
    let output_handle = match get_handle_for_protocol:: <Output>() {
        Ok(h) => h,
        Err(err) => return err.status(),
    };
    let output_protocol: boot::ScopedProtocol<Output> = match boot::open_protocol_exclusive:: <Output>(output_handle) {
        Ok(p) => p,
        Err(err) => return err.status()
    };
    let input_handle = match get_handle_for_protocol:: <Input>() {
        Ok(h) => h,
        Err(err) => return err.status(),
    };
    let mut input_protocol: boot::ScopedProtocol<Input> = match boot::open_protocol_exclusive:: <Input>(input_handle) {
        Ok(p) => p,
        Err(err) => return err.status()
    };

    //
    // Parse args
    //
    let args:Vec<String> = match loaded_image_protocol.load_options_as_cstr16() {
        Ok(s) => s.to_string().split_whitespace().map(|s| s.to_string()).collect(),
        Err(_err) => return Status::ABORTED,
    };

    if args.len() != 2 {
        print!("argments error");
    }

    let mut var_name_utf16: Vec<u16> = args[1].encode_utf16().collect();
    var_name_utf16.push(0);
    let var_name: &CStr16;
    unsafe {
        var_name = CStr16::from_u16_with_nul_unchecked(&var_name_utf16);
    }

    let vendor_guid = match Guid::try_parse(&args[2]) {
        Ok(g) => VariableVendor(g),
        Err(_e) => return Status::INVALID_PARAMETER,
    };

    let attrs: VariableAttributes = VariableAttributes::empty();

    let _ = make_test_variable();

    let mut var_info:variable::VariableInfo = variable::VariableInfo {
        name: var_name,
        guid: vendor_guid,
        attr: attrs,
        size: 0,
        data: Vec::new(),
    };

    let _ = var_info.init();



    let mut ev: Event;

    let mut ef = editor_info::EditorInfo::new(output_protocol, var_info);

    let _ = ef.output_protocol.set_color(Color::Yellow, Color::Black);
    let _ = ef.output_protocol.clear();

    ef.write_all_area();

    loop {
        if ef.need_rewrite {
            ef.write_all_area();
        }

        // update cursor pos
        let current_cursor = ef.bin_area.cursor_pos();  // ef.active_area.cursor_pos()
        let _ = ef.output_protocol.set_cursor_position(current_cursor[0], current_cursor[1]);
        let _ = ef.output_protocol.enable_cursor(true);

        unsafe {
            ev = input_protocol.wait_for_key_event().unwrap().unsafe_clone();
        }
        let _ = uefi::boot::wait_for_event(&mut [ev]);

        if let Ok(Some(key)) = input_protocol.read_key() {
            if ef.input_handle(key) == Status::ABORTED {
                break;
            }
        }
    }
    let _ = ef.output_protocol.clear();

    Status::SUCCESS
}
