use ctor::ctor;
use std::ffi::CString;
use std::os::raw::c_char;
use std::mem::transmute;
use std::time::Duration;

extern "C" {
    pub fn _dyld_get_image_vmaddr_slide(image_index: u32) -> usize;
}

#[repr(u8)]
enum Colours {
    Regular,
    Info,
    Warn,
    Error
}

 fn entry() {
    // address is for version-c7d5978a8bea4dfd
    let print = unsafe { 
        let intermediate_ptr = (_dyld_get_image_vmaddr_slide(0) + 0x100b47aaa) as *const ();
        transmute::<*const (), extern "C" fn (Colours, *const c_char, ...) -> ()>(intermediate_ptr)
    };

    let mut print_msg = CString::new("this is white").unwrap();
    print(Colours::Regular, print_msg.as_ptr());
    std::thread::sleep(Duration::from_millis(100));

    print_msg = CString::new("this is blue").unwrap();
    print(Colours::Info, print_msg.as_ptr());
    std::thread::sleep(Duration::from_millis(100));

    print_msg = CString::new("this is yellow").unwrap();
    print(Colours::Warn, print_msg.as_ptr());
    std::thread::sleep(Duration::from_millis(100));

    print_msg = CString::new("this is red").unwrap();
    print(Colours::Error, print_msg.as_ptr());
}

#[ctor]
unsafe fn constructor() {
    std::thread::spawn(move || entry());
}