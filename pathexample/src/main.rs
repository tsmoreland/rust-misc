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

use std::path::Path;

//
// simple sample playground app to work with std::path::Path a bit, to see how to get directory from path 
//
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() < 2 {
        println!("insufficient arguments");
        return;
    }

    let arg = arguments[1].as_str();
    let path = Path::new(arg);

    println!("Argument: {}", arg);

    let parent = match path.parent() {
        Some(parent_path) => parent_path.as_os_str().to_str().unwrap(),
        None => "",
    };
    println!("parent directory: {}", parent);

    if path.is_file() {
        println!("filename: {}", path.file_name().unwrap().to_os_string().into_string().unwrap());

    } else if path.is_dir() {
        let filename = path.file_name();
        let filename_str = filename.unwrap_or_default().to_os_string().into_string().unwrap_or_default();

        println!("directory: {}", filename_str);

    } else {
        println!("path (as string): {}", path.file_name().unwrap().to_os_string().into_string().unwrap());
    }
}
