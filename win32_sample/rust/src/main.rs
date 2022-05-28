//
// Copyright © 2022 Terry Moreland
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

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
