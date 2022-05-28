use std::slice;
use windows::core::PWSTR;
use windows::Win32::Foundation::BOOLEAN;
use windows::Win32::Security::Authentication::Identity::{EXTENDED_NAME_FORMAT, GetUserNameExW};

fn main() {

    let mut buffer_size = 257_u32;
    let mut buffer = Vec::<u16>::with_capacity(buffer_size as usize);
    let buffer_pwstr = PWSTR(buffer.as_mut_ptr());

    let name_display = EXTENDED_NAME_FORMAT(3);

    let result = unsafe { GetUserNameExW(name_display, buffer_pwstr, &mut buffer_size) };

    if result != BOOLEAN(1) {
        println!("Failure");
        return;
    }

    let buffer = unsafe { slice::from_raw_parts(buffer_pwstr.0, buffer_size as usize) };
    let user_name = String::from_utf16_lossy(buffer);

    println!("Success: {} with length {}", user_name, buffer_size);
}
