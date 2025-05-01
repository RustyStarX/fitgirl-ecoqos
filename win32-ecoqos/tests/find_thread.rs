use std::{sync::mpsc::channel, thread::sleep, time::Duration};

use win32_ecoqos::utils::{Thread, Threads};
use windows::Win32::System::Threading::GetCurrentThreadId;

#[test]
fn find_thread_by_name() -> Result<(), wmi::WMIError> {
    let conn = Threads::try_new()?;
    let (tx, rx) = channel();
    let _ = std::thread::Builder::new()
        .name("mythread".to_owned())
        .spawn(move || {
            let tid = unsafe { GetCurrentThreadId() };
            let _ = tx.send(tid);
            loop {
                sleep(Duration::from_secs(5));
            }
        });
    let thread_id = rx.recv().expect("failed to retrieve thread id");
    let threads = conn.find_thread_by_name("mythread", false)?;

    assert_eq!(
        threads.collect::<Vec<_>>(),
        vec![Thread {
            thread_id,
            thread_name: "mythread".into()
        }]
    );

    Ok(())
}

#[tokio::test]
async fn async_find_thread_by_name() -> Result<(), wmi::WMIError> {
    let conn = Threads::try_new()?;
    let (tx, rx) = channel();
    let _ = std::thread::Builder::new()
        .name("myasyncthread".to_owned())
        .spawn(move || {
            let tid = unsafe { GetCurrentThreadId() };
            let _ = tx.send(tid);
            loop {
                sleep(Duration::from_secs(5));
            }
        });
    let thread_id = rx.recv().expect("failed to retrieve thread id");
    let threads = conn
        .async_find_thread_by_name("myasyncthread", false)
        .await?;

    assert_eq!(
        threads.collect::<Vec<_>>(),
        vec![Thread {
            thread_id,
            thread_name: "myasyncthread".into()
        }]
    );

    Ok(())
}
