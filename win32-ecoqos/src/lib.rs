use std::ffi::c_void;

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Threading::{
        IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, OpenProcess, PROCESS_CREATION_FLAGS,
        PROCESS_INFORMATION_CLASS, PROCESS_POWER_THROTTLING_STATE, PROCESS_SET_INFORMATION,
        ProcessPowerThrottling, SetPriorityClass, SetProcessInformation,
    },
};

pub use windows_result;

mod preset;
use preset::{THROTTLE, UNTHROTTLE};

unsafe fn toggle_efficiency_mode_impl(
    hprocess: HANDLE,
    processinformation: *const c_void,
    processinformationclass: PROCESS_INFORMATION_CLASS,
    processinformationsize: u32,
    dwpriorityclass: PROCESS_CREATION_FLAGS,
) -> Result<(), windows_result::Error> {
    unsafe {
        SetProcessInformation(
            hprocess,
            processinformationclass,
            processinformation,
            processinformationsize,
        )?;
        SetPriorityClass(hprocess, dwpriorityclass)?;
    }

    Ok(())
}

/// Toggle efficiency mode of a process, by it's PID.
///
/// SAFETY: see [`toggle_efficiency_mode_handle`]
pub unsafe fn toggle_efficiency_mode(pid: u32, enable: bool) -> Result<(), windows_result::Error> {
    let hprocess = unsafe { OpenProcess(PROCESS_SET_INFORMATION, false, pid)? };

    let result = unsafe { toggle_efficiency_mode_handle(hprocess, enable) };
    let close_handle = unsafe { CloseHandle(hprocess) };
    close_handle.or(result)
}

/// Toggle efficiency mode of a process, by a `HANDLE`.
///
/// SAFETY: you must not call failable Win32 APIs in other threads,
///
/// or it may override [`GetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror),
/// thus, you will get wrong [`Error`](windows_result::Error).
pub unsafe fn toggle_efficiency_mode_handle(
    hprocess: HANDLE,
    enable: bool,
) -> Result<(), windows_result::Error> {
    let new_state = if enable { THROTTLE } else { UNTHROTTLE };

    let processinformationclass = ProcessPowerThrottling;
    let processinformation = &new_state as *const _ as *const c_void;
    let processinformationsize = size_of::<PROCESS_POWER_THROTTLING_STATE>() as u32;

    let dwpriorityclass = if enable {
        IDLE_PRIORITY_CLASS
    } else {
        NORMAL_PRIORITY_CLASS
    };

    unsafe {
        toggle_efficiency_mode_impl(
            hprocess,
            processinformation,
            processinformationclass,
            processinformationsize,
            dwpriorityclass,
        )
    }
}
