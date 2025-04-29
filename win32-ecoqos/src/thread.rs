use std::ffi::c_void;

use crate::preset::{THREAD_THROTTLE, THREAD_UNTHROTTLE};
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Threading::{
        OpenThread, SetThreadInformation, SetThreadPriority, THREAD_INFORMATION_CLASS,
        THREAD_POWER_THROTTLING_STATE, THREAD_PRIORITY, THREAD_PRIORITY_IDLE,
        THREAD_PRIORITY_NORMAL, THREAD_SET_INFORMATION, ThreadPowerThrottling,
    },
};

unsafe fn toggle_efficiency_mode_impl(
    hthread: HANDLE,
    threadinformation: *const c_void,
    threadinformationclass: THREAD_INFORMATION_CLASS,
    threadinformationsize: u32,
    npriority: THREAD_PRIORITY,
) -> Result<(), windows_result::Error> {
    unsafe {
        SetThreadInformation(
            hthread,
            threadinformationclass,
            threadinformation,
            threadinformationsize,
        )?;
        SetThreadPriority(hthread, npriority)?;
    }

    Ok(())
}

/// Toggle efficiency mode of a thread, by a thread_id.
///
/// WARN: [`Thread::id()`](https://doc.rust-lang.org/std/thread/struct.Thread.html#method.id)
/// is entirely unrelated to underlying thread ID.
///
/// SAFETY: you must not call failable Win32 APIs in other threads,
///
/// or it may override [`GetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror),
/// thus, you will get wrong [`Error`](windows_result::Error).
pub unsafe fn toggle_efficiency_mode(
    thread_id: u32,
    enable: bool,
) -> Result<(), windows_result::Error> {
    let hthread = unsafe { OpenThread(THREAD_SET_INFORMATION, false, thread_id)? };
    let result = unsafe { toggle_efficiency_mode_handle(hthread, enable) };
    let close_handle = unsafe { CloseHandle(hthread) };

    close_handle.or(result)
}

/// Toggle efficiency mode of a thread, by a [`HANDLE`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HANDLE.html).
///
/// [`GetCurrentThread`]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.GetCurrentThread.html
/// The handle returned by [`GetCurrentThread`] have a `THREAD_ALL_ACCESS`.
///
/// You must enable [`THREAD_SET_INFORMATION`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/constant.THREAD_SET_INFORMATION.html)
/// access flag on the handle to apply EcoQoS throttle.
///
/// SAFETY: you must not call failable Win32 APIs in other threads,
///
/// or it may override [`GetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror),
/// thus, you will get wrong [`Error`](windows_result::Error).
pub unsafe fn toggle_efficiency_mode_handle(
    hthread: HANDLE,
    enable: bool,
) -> Result<(), windows_result::Error> {
    let new_state = if enable {
        THREAD_THROTTLE
    } else {
        THREAD_UNTHROTTLE
    };

    let threadinformationclass = ThreadPowerThrottling;
    let threadinformation = &new_state as *const _ as *const c_void;
    let threadinformationsize = size_of::<THREAD_POWER_THROTTLING_STATE>() as u32;

    let npriority = if enable {
        THREAD_PRIORITY_IDLE
    } else {
        THREAD_PRIORITY_NORMAL
    };

    unsafe {
        toggle_efficiency_mode_impl(
            hthread,
            threadinformation,
            threadinformationclass,
            threadinformationsize,
            npriority,
        )
    }
}
