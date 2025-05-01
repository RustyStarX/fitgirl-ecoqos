use std::{ffi::OsString, os::windows::ffi::OsStringExt};

use windows::Win32::{
    Foundation::CloseHandle,
    System::{
        ProcessStatus::{EnumProcesses, GetModuleBaseNameW},
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};

/// get process name, return UTF-16LE encoded String
pub fn get_process_name(
    process_id: u32,
    name_max_len: Option<u32>,
) -> windows_result::Result<OsString> {
    let mut process_name = Vec::with_capacity(name_max_len.unwrap_or(1024) as usize);

    unsafe {
        let hprocess = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        )?;
        process_name.set_len(1024);

        let new_len = GetModuleBaseNameW(hprocess, None, process_name.as_mut_slice());
        process_name.set_len(new_len as usize);

        let _ = CloseHandle(hprocess);
    }

    Ok(OsString::from_wide(&process_name))
}

/// List process id
///
/// Due to win32 API limitation, it's hard to find number of current processes.
pub fn list_process_id(expected_len: u32) -> windows_result::Result<Vec<u32>> {
    let mut real_length = 0_u32;
    let mut pid_list = Vec::with_capacity(expected_len as usize);
    let spare_cap = pid_list.spare_capacity_mut();

    unsafe {
        EnumProcesses(
            spare_cap.as_mut_ptr() as *mut u32,
            (size_of::<u32>() * spare_cap.len()) as u32,
            &mut real_length as *mut u32,
        )?
    };

    unsafe {
        pid_list.set_len(real_length as usize / size_of::<u32>());
    }

    Ok(pid_list)
}

/// List all process id
///
/// Due to win32 API limitation, this actually would make length of a `Vec<u32>` two times,
/// until real length < allocated length.
///
/// The assume will start with `2^N` length.
pub fn list_all_process_id<const N: u32>() -> windows_result::Result<Vec<u32>> {
    let mut expected_len = 2_u32.pow(N);

    let mut result = list_process_id(expected_len)?;

    while result.len() == expected_len as usize {
        expected_len *= 2;
        result = list_process_id(expected_len)?;
    }

    Ok(result)
}
