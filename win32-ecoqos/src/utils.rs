use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use windows::Win32::{
    Foundation::CloseHandle,
    System::Threading::{GetThreadDescription, OpenThread, THREAD_QUERY_LIMITED_INFORMATION},
};
use wmi::{COMLibrary, FilterValue, WMIConnection};

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename = "Win32_Thread")]
struct Win32Thread {
    /// thread id
    handle: String,
}

/// Thread continer
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Thread {
    pub thread_id: u32,
    pub thread_name: String,
}

/// WMI connection
pub struct Threads {
    wmi: WMIConnection,
}

impl Threads {
    pub fn try_new() -> Result<Self, wmi::WMIError> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con)?;

        Ok(Self { wmi: wmi_con })
    }

    pub fn find_thread_by_name(
        &self,
        name: impl AsRef<str>,
        full_match: bool,
    ) -> Result<Vec<Thread>, wmi::WMIError> {
        let pid = std::process::id();

        let mut filter = HashMap::new();
        filter.insert("ProcessHandle".to_string(), FilterValue::Number(pid as i64));

        let threads = self.wmi.filtered_query::<Win32Thread>(&filter)?;
        Ok(filter_threads(threads, full_match, name.as_ref()))
    }

    pub async fn async_find_thread_by_name(
        &self,
        name: impl AsRef<str>,
        full_match: bool,
    ) -> Result<Vec<Thread>, wmi::WMIError> {
        let pid = std::process::id();

        let mut filter = HashMap::new();
        filter.insert("ProcessHandle".to_string(), FilterValue::Number(pid as i64));

        let threads = self
            .wmi
            .async_filtered_query::<Win32Thread>(&filter)
            .await?;
        Ok(filter_threads(threads, full_match, name.as_ref()))
    }
}

fn filter_threads(threads: Vec<Win32Thread>, full_match: bool, name: &str) -> Vec<Thread> {
    threads
        .into_iter()
        .filter_map(|Win32Thread { handle }| unsafe {
            let thread_id = handle.trim_matches('\"').parse().ok()?;
            let hthread = OpenThread(THREAD_QUERY_LIMITED_INFORMATION, false, thread_id).ok()?;
            let description = GetThreadDescription(hthread);
            let _ = CloseHandle(hthread);
            let thread_name = description.ok()?;
            let thread_name = thread_name.to_string().ok()?;
            if full_match && &thread_name == name || !full_match && thread_name.contains(name) {
                Some(Thread {
                    thread_id,
                    thread_name,
                })
            } else {
                None
            }
        })
        .collect()
}
