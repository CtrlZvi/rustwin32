// Wrappers around items provided by Winbase.h, Kernel32.lib, and Kernel32.dll
extern crate kernel32;

use winapi;
use user32::*;

use std;
use std::os::windows::ffi::OsStringExt;

pub fn get_module_filename(module: Option<&Module>) -> Result<String, std::io::Error> {
    // TODO(zeffron 2016-08-19): Figure out how to pick the correct size.
    let mut file_name: [winapi::WCHAR; 512] = unsafe { std::mem::uninitialized() };
    let size = unsafe {
        kernel32::GetModuleFileNameW(
            match module {
                Some(module) => module.handle,
                None => std::ptr::null_mut(),
            },
            file_name.as_mut_ptr(),
            file_name.len() as u32,
        )
    };
    match size {
        0 => Err(std::io::Error::last_os_error()),
        _ => Ok(std::ffi::OsString::from_wide(file_name.iter().take(std::cmp::min(size as usize + 1, file_name.len())).take_while(|c| **c != 0).map(|c| *c).collect::<Vec<u16>>().as_slice()).into_string().unwrap())
    }
}