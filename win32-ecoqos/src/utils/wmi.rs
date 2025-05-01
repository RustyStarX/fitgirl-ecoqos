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

/// Information from a win32 thread
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Thread {
    /// win32 thread_id
    pub thread_id: u32,
    /// win32 thread description
    pub thread_name: String,
}

/// WMI connection
pub struct Threads {
    wmi: WMIConnection,
}

impl Threads {
    /// try to make WMI connection
    pub fn try_new() -> Result<Self, wmi::WMIError> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con)?;

        Ok(Self { wmi: wmi_con })
    }

    /// find a thread by it's name
    ///
    /// - `full_match`: set to false for contains check, otherwise PartialEq
    ///
    /// ```rust
    /// use win32_ecoqos::utils::Threads;
    /// use std::time::Duration;
    ///
    /// let _ = std::thread::Builder::new()
    ///     .name("mythread".into())
    ///     .spawn(|| loop {
    ///         std::thread::sleep(Duration::from_secs(60));
    ///     });
    ///
    /// let threads = Threads::try_new().expect("failed to make wmi connection");
    ///
    /// assert!(threads
    ///     .find_thread_by_name("mythread", true)
    ///     .expect("failed to find thread")
    ///     .next()
    ///     .is_some());
    /// assert!(threads
    ///     .find_thread_by_name("my-thread", true)
    ///     .expect("failed to find thread")
    ///     .next()
    ///     .is_none());
    /// ```
    pub fn find_thread_by_name<'a>(
        &self,
        name: &'a str,
        full_match: bool,
    ) -> Result<impl Iterator<Item = Thread> + 'a, wmi::WMIError> {
        let pid = std::process::id();

        let mut filter = HashMap::new();
        filter.insert("ProcessHandle".to_string(), FilterValue::Number(pid as i64));

        let threads = self.wmi.filtered_query::<Win32Thread>(&filter)?;
        Ok(filter_threads(threads, full_match, name.as_ref()))
    }

    /// find a thread by it's name, async version
    ///
    /// - `full_match`: set to false for contains check, otherwise PartialEq
    pub async fn async_find_thread_by_name<'a>(
        &self,
        name: &'a str,
        full_match: bool,
    ) -> Result<impl Iterator<Item = Thread> + 'a, wmi::WMIError> {
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

fn filter_threads<'a>(
    threads: Vec<Win32Thread>,
    full_match: bool,
    name: &'a str,
) -> impl Iterator<Item = Thread> + 'a {
    threads
        .into_iter()
        .filter_map(move |Win32Thread { handle }| unsafe {
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
}
