
extern crate alloc;

use core::ffi::c_void;
use core::mem::MaybeUninit;
use uefi::proto::unsafe_protocol;
use uefi::guid;
use uefi::proto::console::text::ScanCode;
use uefi::{Guid, Status};

#[derive(Debug)]
#[repr(C)]
pub struct SimpleTextInputEx {
    reset:
        extern "efiapi" fn(this: *mut SimpleTextInputEx, extended_verification: bool) -> Status,

    read_key_stroke_ex:
        extern "efiapi" fn(this: *mut SimpleTextInputEx, key_data: *mut EfiKeyData) -> Status,
    
    pub wait_for_key_ex: uefi_raw::Event,


    set_state:
        extern "efiapi" fn(this: *mut SimpleTextInputEx, toggle_state: *mut KeyToggleState) -> Status,

    register_key_notify: extern "efiapi" fn(
        this: *mut SimpleTextInputEx,
        key_data: *const EfiKeyData,
        key_notify_func: extern "efiapi" fn(*const EfiKeyData) -> Status,
        notify_handle: *mut *mut c_void,
    ) -> Status,

    unregister_key_notify:
        extern "efiapi" fn(this: *mut SimpleTextInputEx, notify_handle: *mut c_void) -> Status,
}

impl SimpleTextInputEx {
    pub const GUID: Guid = guid!("DD9E7534-7762-4698-8C14-F58517A625AA");

}

#[derive(Debug)]
#[repr(transparent)]
#[unsafe_protocol(SimpleTextInputEx::GUID)]
pub struct InputEx(SimpleTextInputEx);


#[repr(C)]
#[derive(Clone, Copy)]
pub struct EfiInputKey {
    pub scan_code: ScanCode,
    pub unicode_char: u16,
}

use bitflags::bitflags;
bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]

    pub struct KeyShiftState: u32 {
        const RIGHT_SHIFT   = 0x0000_0001;
        const LEFT_SHIFT    = 0x0000_0002;
        const RIGHT_CONTROL = 0x0000_0004;
        const LEFT_CONTROL  = 0x0000_0008;
        const RIGHT_ALT     = 0x0000_0010;
        const LEFT_ALT      = 0x0000_0020;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct KeyToggleState {
    key_toggle_state: u8,
}

/// EFI_KEY_STATE
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EfiKeyState {
    pub key_shift_state: KeyShiftState,
    pub key_toggle_state: KeyToggleState,
}


/// EFI_KEY_DATA
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EfiKeyData {
    pub key: EfiInputKey,
    pub key_state: EfiKeyState,
}

impl InputEx {
    pub fn read_key(&mut self) -> Option<EfiKeyData> {
        let mut key = MaybeUninit::<EfiKeyData>::uninit();

        match (self.0.read_key_stroke_ex)(&mut self.0, key.as_mut_ptr())  {
            Status::NOT_READY => None,
            //other => other.to_result_with_val(|| Some(unsafe { key.assume_init() }.into())),
            _ => Some(unsafe { key.assume_init() }.into()),
        }
    }

    #[must_use]
    pub fn wait_for_key_event(&self) -> Option<uefi::Event> {
        unsafe { uefi::Event::from_ptr(self.0.wait_for_key_ex) }
    }
}


