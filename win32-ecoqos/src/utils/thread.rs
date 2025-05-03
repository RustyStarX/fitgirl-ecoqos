use std::{
    ffi::{OsStr, OsString},
    os::windows::ffi::OsStringExt,
};

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
        },
        Threading::{GetThreadDescription, OpenThread, THREAD_QUERY_LIMITED_INFORMATION},
    },
};

#[derive(Debug, PartialEq, Eq)]
/// process information from snapshot.
pub struct Thread {
    pub thread_id: u32,
    pub owner_process_id: u32,
}

#[derive(Debug)]
pub struct Threads {
    snapshot: HANDLE,
    last_entry: Option<THREADENTRY32>,
}

impl Drop for Threads {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.snapshot) };
    }
}

impl Thread {
    pub fn get_name(&self) -> windows_result::Result<OsString> {
        unsafe {
            let hthread = OpenThread(THREAD_QUERY_LIMITED_INFORMATION, false, self.thread_id)?;
            let description = GetThreadDescription(hthread)?;

            Ok(OsString::from_wide(description.as_wide()))
        }
    }
}

impl Threads {
    pub fn try_new() -> windows_result::Result<Self> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, std::process::id()) }?;
        Ok(Self {
            snapshot,
            last_entry: None,
        })
    }

    pub fn find_thread_by_name<'a>(
        self,
        thread_name: &'a OsStr,
    ) -> impl Iterator<Item = Thread> + 'a {
        let process_id = std::process::id();
        self.filter(move |t| t.owner_process_id == process_id)
            .filter(|t| t.get_name().as_deref() == Ok(thread_name))
    }
}

impl Iterator for Threads {
    type Item = Thread;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.last_entry.is_none();
        let mut entry = self.last_entry.take().unwrap_or(THREADENTRY32 {
            dwSize: size_of::<THREADENTRY32>() as u32,
            ..Default::default()
        });

        unsafe {
            if first {
                Thread32First(self.snapshot, &mut entry as *mut _)
            } else {
                Thread32Next(self.snapshot, &mut entry as *mut _)
            }
        }
        .ok()
        .map(|_| entry)
        .inspect(|entry| self.last_entry = Some(*entry))
        .map(
            |THREADENTRY32 {
                 th32ThreadID,
                 th32OwnerProcessID,
                 ..
             }| {
                Thread {
                    thread_id: th32ThreadID,
                    owner_process_id: th32OwnerProcessID,
                }
            },
        )
    }
}
